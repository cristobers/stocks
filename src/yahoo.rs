use reqwest::{get, StatusCode};
use serde_json::Value;
use serde::Serialize;
use std::{fs, str::FromStr};

use crate::database::timestamp;

#[derive(Serialize, Debug, Clone)]
pub struct Stock {
    pub name:             String,
    pub market_price:     f64,
    pub market_day_high:  f64,
    pub market_day_low:   f64,
    pub last_get_request: u64
}

pub fn bad_stock() -> Stock {
     Stock {
        name:             String::from("None"),
        market_price:     0.0,
        market_day_high:  0.0,
        market_day_low:   0.0,
        last_get_request: 0,
    }
}

pub async fn parse_json(stock_json: &str) -> Stock {
    let json : Value = serde_json::from_str(&stock_json).unwrap();

    if json["chart"]["result"][0]["meta"]["instrumentType"] != "EQUITY" {
        return bad_stock();
    }

    let name = json["chart"]["result"][0]["meta"]["symbol"]
        .to_string();

    let market_price = json["chart"]["result"][0]["meta"]["regularMarketPrice"]
        .as_f64()
        .unwrap();

    let market_day_high = json["chart"]["result"][0]["meta"]["regularMarketDayHigh"]
        .as_f64()
        .unwrap();

    let market_day_low = json["chart"]["result"][0]["meta"]["regularMarketDayLow"]
        .as_f64()
        .unwrap();

    Stock {
        name: serde_json::from_str(&name).unwrap(),
        market_price,
        market_day_high,
        market_day_low,
        last_get_request:     timestamp(),
    }
}

fn create_headers() -> reqwest::header::HeaderMap {

    let config = serde_json::json!(fs::read_to_string("config.json").unwrap());
    let user_agent = config["user-agent"].to_string();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_str(
            &user_agent
        ).unwrap()
    );
    headers
}

pub async fn get_req(stock_name: &str) -> Stock {
    // get stock should only make a req if the entry doesnt exist in the db.
    // dont you DARE go over that ruler
    let url = String::from(
        format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}?region=GB&lang=en-GB",
            stock_name
        )
    );

    println!("{:?}", url);

    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .headers(create_headers())
        .send()
        .await.unwrap();

    if resp.status() == StatusCode::NOT_FOUND {
        println!("Bad stock name found: {}", &stock_name);
        return bad_stock();
    }

    let text = resp.text().await.unwrap();
    // dbg!(&text);
    parse_json(&text).await
}
