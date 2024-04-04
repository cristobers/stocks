use rusqlite::{Connection};
use crate::yahoo::Stock;
use std::path::Path;

pub fn connect(name: &str) -> Connection {
    Connection::open(name)
        .unwrap()
}

pub async fn insert_stock(data : Stock, db_name: &str) {
    let conn = connect(db_name);
    conn.execute(
        "REPLACE INTO 
        stocks(stock_name, stock_price, stock_day_high, stock_day_low)
        VALUES (?1, ?2, ?3, ?4)
        ", (data.name, data.market_price, data.market_day_high, data.market_day_low),
    ).unwrap();
}

pub fn create_stocks() {
    if Path::new("stocks.db").exists() {
        return;
    } else {
        let conn = connect("stocks.db");
        conn.execute(
            "CREATE TABLE stocks (
                stock_name     TEXT PRIMARY KEY,
                stock_price    FLOAT,
                stock_day_high FLOAT,
                stock_day_low  FLOAT
            )",
            (),
        ).unwrap();
    }
}