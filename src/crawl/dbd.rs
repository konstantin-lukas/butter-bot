use scraper::{Html, Selector};
use serenity::futures::future;
use regex::Regex;
use std::collections::HashMap;


pub async fn run() {
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
                        codes.insert(
                            String::from(res[0].trim()),
                            String::from(res[1].trim())
                        );
                    }
                }
            }

        }
    }

    for code in codes {
        println!("{} - {}",code.0,code.1);
    }



}