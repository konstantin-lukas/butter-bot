use scraper::{Html, Selector};
use serenity::futures::future;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use chrono::{DateTime, Utc, FixedOffset};
use crate::utils::message::MessageHandler;


pub async fn run(message_handler: &MessageHandler) {
    let pcg = reqwest::get("https://www.pcgamesn.com/dead-by-daylight/dbd-codes");
    let rps = reqwest::get("https://www.rockpapershotgun.com/dead-by-daylight-codes-list");
    let dex = reqwest::get("https://www.dexerto.com/dead-by-daylight/how-to-redeem-dead-by-deadlight-codes-1664016/");

    let results = future::join_all([pcg, rps, dex]).await;

    let mut ok_results = Vec::new();
    for result in results {
        if let Ok(response) = result {
            ok_results.push(response.text())
        }
    }
    let results = future::join_all(ok_results).await;

    // MATCH COLON AND DIFFERENT TYPES OF DASHES
    let pattern = Regex::new(
        r"[\u003A\u2013\u002D\u05BE\u1806\u2010\u2015\u2E3A\u2E3B\uFE58\uFE63\uFF0D]"
    ).unwrap();

    let mut codes: HashMap<String, String> = HashMap::new();
    let mut new_codes: HashMap<String, String> = HashMap::new();

    match fs::read_to_string("dbd_codes.csv") {
        Ok(file) => {
            let lines = file.split('\n');
            for line in lines {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() == 2 {
                    codes.insert(
                        String::from(parts[0]),
                        String::from(parts[1])
                    );
                }
            }

        },
        Err(e) => {
            message_handler.dm_admin(
                format!("An error occurred trying to read DBD codes from file: ```\n{e}\n```").as_str()
            ).await;
        }
    }

    for result in results {
        if let Ok(text) = result {
            let document = Html::parse_document(&text);
            let ul_selector = Selector::parse("h2 ~ ul").unwrap();
            let li_selector = Selector::parse("li").unwrap();
            if let Some(ul) = document.select(&ul_selector).next() {
                for element in ul.select(&li_selector) {
                    let text_content: String = element.text().collect();
                    let res: Vec<String> = pattern.split(text_content.as_str()).map(|s| s.to_string()).collect();
                    if res.len() == 2 {
                        let key = String::from(res[0].trim());
                        let value = String::from(res[1].trim());
                        if !codes.contains_key(key.as_str()) {
                            codes.insert(key.clone(), value.clone());
                            new_codes.insert(key, value);
                        }
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
        let mut message = String::from("I found some fresh new codes since my last crawl! 🩸\n\n");
        for new_code in new_codes {
            let utc_now: DateTime<Utc> = Utc::now();
            let mut formatted_date = None;
            if let Some(gmt1_offset) = FixedOffset::east_opt(3600) {
                let gmt1_now = utc_now.with_timezone(&gmt1_offset);
                formatted_date = Some(gmt1_now.format("%Y/%m/%d").to_string());
            }
            if let Some(date) = formatted_date {
                message.push_str(
                    format!("- `{}`: {} ({})\n", new_code.0, new_code.1,date).as_str()
                );
            } else {
                message.push_str(
                    format!("- `{}`: {}\n", new_code.0, new_code.1).as_str()
                );
            }
        }
        message_handler.post_to_dbd_channel(message.as_str()).await;
    }

    match fs::write("dbd_codes.csv", file_content) {
        Ok(_) => { },
        Err(e) => {
            message_handler.dm_admin(
                format!("An error occurred trying to write DBD codes to file: ```\n{e}\n```").as_str()
            ).await;
        }
    }

}