use rusqlite::{Connection};
use crate::yahoo::Stock;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn should_we_pull_new_prices(stock : &Stock) -> bool {
    let curr_time = timestamp();
    if (curr_time - stock.last_get_request) >= 3600 {
        return true;
    }
    false
}

fn connect(name: &str) -> Connection {
    Connection::open(name)
        .unwrap()
}

pub fn get(stock_name : &str) -> Stock {
    let conn = connect("stocks.db");
    let mut stmt = conn.prepare(
        "SELECT *
        FROM stocks
        WHERE stock_name = ?1",
    ).unwrap();
    let mut rows = stmt.query(rusqlite::params![stock_name]).unwrap();
    let test = rows.next().unwrap();
    if test.is_none() {
        Stock {
            name:             String::from("None"),
            market_price:     0.0,
            market_day_high:  0.0,
            market_day_low:   0.0,
            last_get_request: 0,
        }
    } else {
        Stock {
            name:             test.unwrap().get(0).unwrap(),
            market_price:     test.unwrap().get(1).unwrap(),
            market_day_high:  test.unwrap().get(2).unwrap(),
            market_day_low:   test.unwrap().get(3).unwrap(),
            last_get_request: test.unwrap().get(4).unwrap(),
        }
    }
}

pub fn get_names() -> Vec<String> {
    let conn = connect("stocks.db");

    let mut stmt = conn.prepare(
        "SELECT stock_name FROM stocks",
    ).unwrap();

    let mut rows = stmt.query([]).unwrap();
    let mut names = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        names.push(row.get(0).unwrap());
    }
    names
}

pub async fn insert_stock(data : Stock, db_name: &str) {
    let conn = connect(db_name);
    conn.execute(
        "REPLACE INTO 
        stocks(stock_name, stock_price, stock_day_high, stock_day_low, last_get_request)
        VALUES (?1, ?2, ?3, ?4, ?5)
        ", (
            data.name, 
            data.market_price, 
            data.market_day_high, 
            data.market_day_low, 
            timestamp()
        ),
    ).unwrap();
}

pub fn create_table(sql_command : &str) {
    let conn = connect("stocks.db");
    conn.execute(
        &sql_command,
        (),
    ).unwrap();
}

pub fn create_stocks() {
    create_table(
        "CREATE TABLE stocks (
            stock_name       TEXT PRIMARY KEY,
            stock_price      FLOAT,
            stock_day_high   FLOAT,
            stock_day_low    FLOAT,
            last_get_request BIGINT
        )"
    );
}

pub fn create_users_to_stocks() {
    create_table(
        "CREATE TABLE users_to_stocks (
            user_id BIGINT,
            stock_name TEXT,
            FOREIGN KEY (user_id)    REFERENCES users(user_id),
            FOREIGN KEY (stock_name) REFERENCES stocks(stock_name)
        )"
    );
}

pub fn create_users() {
    create_table(
        "CREATE TABLE users (
            user_id BIGINT PRIMARY KEY,
            money   FLOAT 
        )"
    );
}