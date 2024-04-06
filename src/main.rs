use tokio;
use std::net::{TcpListener, TcpStream};
use crate::yahoo::Stock;
use futures;
use std::io::Write;
use std::io::Read;

mod yahoo;
mod database;

async fn initialise() {
    database::create_stocks();
    let stocks: Vec<&str> = [
        "GME",
        "NVDA",
        "AMD",
        "INTC",
        "BTC-USD",
        "TSLA",
        "NFLX",
        "META",
        "0R3E.L"
    ].to_vec();

    for stock in stocks.into_iter() {
        let db_query = database::get(&stock);

        // if it exists in the db, print it
        if db_query.name != "None" {
            println!("{:?}", db_query);
            continue;
        }

        let yahoo_req = yahoo::get_req(&stock).await;
        if yahoo_req.name != "None" {
            println!("Adding {} to the database", &stock);
            database::insert_stock(yahoo_req, "stocks.db").await;
        } else {
            println!("{} Wasn't a recognised stock name", &stock);
        }
    }
}

async fn buy_stock() {
    todo!("buying :3");
}

async fn sell_stock() {
    todo!("selling :3");
}

async fn query_stock(name: &str, mut conn: &TcpStream) {
    let mut initial_query = database::get(&name);
    if database::should_we_pull_new_prices(&initial_query) {
        println!("Getting new information for: {}", &initial_query.name);
        let updated_stock = yahoo::get_req(&initial_query.name).await;
        database::insert_stock(updated_stock.clone(), "stocks.db").await;
        initial_query = updated_stock;
    }
    let parsed = serde_json::to_string(&initial_query).unwrap();
    conn.write(
        &parsed.as_bytes()
    ).unwrap();
    println!("Sending resonse: {:?}", &parsed);
}

async fn handle_conn (mut conn: &TcpStream) {
    let mut buf : [u8; 32] = [0; 32];

    conn.read(&mut buf).unwrap();
    let stringed = String::from_utf8_lossy(&buf);
    let parse_input : Vec<&str> = stringed.split('\0').collect();

    let full_cmd = parse_input[0];
    let split_cmd : Vec<&str> = full_cmd.split_whitespace().collect();
    let (mut cmd, mut stock, mut amount) = ("", "", "");

    if split_cmd.len() == 3 {
        (cmd, stock, amount) = (split_cmd[0], split_cmd[1], split_cmd[2]);
    } else if split_cmd.len() == 2 {
        (cmd, stock) = (split_cmd[0], split_cmd[1]);
    } else {
        panic!("incorrect number of arguments");
    }

    println!("Command: {} Stock: {} Amount: {}", &cmd, &stock, &amount);

    match cmd {
        "BUY"   => buy_stock().await,
        "SELL"  => sell_stock().await,
        "QUERY" => query_stock(stock, &conn).await,
        _ => todo!("UNKNOWN COMMAND!!!"),
    };
}

async fn get_stocks(time_out: u64) {
    loop {
        let names : Vec<String> = database::get_names();
        println!("Database stock names: {:?}", &names);
        for name in names {
            let stock_struct : Stock = yahoo::get_req(&name).await;
            println!("{:?}", stock_struct);
            database::insert_stock(stock_struct, "stocks.db").await;
        }
        println!("Hello!, sleeping for {} seconds...", time_out);
        tokio::time::sleep(tokio::time::Duration::from_secs(time_out)).await;
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7690")
        .expect("PORT DIDNT OPEN WHY GOT WHY");

    for stream in listener.incoming() {
        let mut curr_stream = stream.unwrap();
        tokio::spawn(async move {
            handle_conn(&mut curr_stream).await;
        });
    }
}