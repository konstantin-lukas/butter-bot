use scraper::{Html, Selector};
use serenity::futures::future;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use chrono::{DateTime, Utc, FixedOffset};
use serenity::all::Http;
use crate::utils::message::{dm_admin, post_to_dbd_channel};


pub async fn run(http: Arc<Http>) {

    // MATCH COLON AND DIFFERENT TYPES OF DASHES
    let pattern = match Regex::new(
        r"[\u003A\u2013\u002D\u05BE\u1806\u2010\u2015\u2E3A\u2E3B\uFE58\uFE63\uFF0D]"
    ) {
        Ok(regex) => regex,
        Err(e) => {
            dm_admin(
                http.clone(),
                format!("An error occurred trying to construct regex: ```\n{e}\n```").as_str()
            ).await;
            return;
        }
    };

    let results = future::join_all([
        reqwest::get("https://www.pcgamesn.com/dead-by-daylight/dbd-codes"),
        reqwest::get("https://www.rockpapershotgun.com/dead-by-daylight-codes-list"),
        reqwest::get("https://www.dexerto.com/dead-by-daylight/how-to-redeem-dead-by-deadlight-codes-1664016/")]
    ).await;

    let results = future::join_all(
        results.into_iter().flatten().map(|x| x.text())
    ).await.into_iter().flatten().collect::<Vec<_>>();

    let mut codes: HashMap<String, String> = HashMap::new();
    let mut new_codes: HashMap<String, String> = HashMap::new();

    let file;
    match fs::read_to_string("dbd_codes.csv") {
        Ok(data) => {
            file = data;
            for line in file.lines() {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() == 2 {
                    codes.insert(
                        parts[0].to_string(),
                        parts[1].to_string()
                    );
                }
            }

        },
        Err(e) => {
            dm_admin(
                http.clone(),
                format!("An error occurred trying to read DBD codes from file: ```\n{e}\n```").as_str()
            ).await;
        }
    }

    for result in results {
        let document = Html::parse_document(&result);
        let ul_selector = Selector::parse("h2 ~ ul").unwrap();
        let li_selector = Selector::parse("li").unwrap();
        if let Some(ul) = document.select(&ul_selector).next() {
            for element in ul.select(&li_selector) {
                let text_content = element.text().collect::<String>();
                let res = pattern
                    .split(text_content.as_str())
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
                if res.len() == 2 {
                    let key = res[0].trim().to_string();
                    let value = res[1].trim().to_string();
                    if !codes.contains_key(&key) {
                        codes.insert(key.clone(), value.clone());
                        new_codes.insert(key, value);
                    }
                }
            }
        }
    }

    let mut file_content = String::new();
    for code in codes {
        file_content.push_str(format!("{}\t{}\n", code.0, code.1).as_str());
    }

    if new_codes.len() > 0 {
        let mut message = String::from("I found some fresh new codes since my last crawl! 🩸\n");
        for (code, desc) in new_codes {
            let utc_now: DateTime<Utc> = Utc::now();
            let mut formatted_date = None;
            if let Some(gmt1_offset) = FixedOffset::east_opt(3600) {
                let gmt1_now = utc_now.with_timezone(&gmt1_offset);
                formatted_date = Some(gmt1_now.format("%Y/%m/%d").to_string());
            }
            if let Some(date) = formatted_date {
                message.push_str(
                    format!("- `{}`: {} ({})\n", code, desc,date).as_str()
                );
            } else {
                message.push_str(
                    format!("- `{}`: {}\n", code, desc).as_str()
                );
            }
        }
        post_to_dbd_channel(http.clone(), message.as_str()).await;
    }

    if let Err(e) = fs::write("dbd_codes.csv", file_content) {
        dm_admin(
            http.clone(),
            format!("An error occurred trying to write DBD codes to file: ```\n{e}\n```").as_str()
        ).await;
    }

}