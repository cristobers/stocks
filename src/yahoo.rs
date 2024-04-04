use reqwest::{get, StatusCode};
use serde_json::{Value};

#[derive(Debug)]
pub struct Stock {
    pub name:            String,
    pub market_price:    f64,
    pub market_day_high: f64,
    pub market_day_low:  f64,
}

pub async fn parse_json(stock_json: &str) -> Stock {
    let json : Value = serde_json::from_str(&stock_json).unwrap();

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
        name: name,
        market_price: market_price,
        market_day_high: market_day_high,
        market_day_low: market_day_low,
    }
}

pub async fn get_stock(stock_name: &str) -> Stock {
    // dont you DARE go over that ruler
    let url = String::from(
        format!(
            "https://query1.finance.yahoo.com/v8/finance/chart/{}?region=GB&lang=en-GB",
            stock_name
        )
    );

    let resp = get(url)
        .await.unwrap();

    if resp.status() == StatusCode::NOT_FOUND {
        println!("Bad stock name found: {}", &stock_name);
        return Stock {
            name: String::from("None"),
            market_price: 0.0,
            market_day_high: 0.0,
            market_day_low: 0.0,
        }
    }

    let text = resp.text().await.unwrap();
    parse_json(&text).await
}