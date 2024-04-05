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

async fn query_stock(name: &str, mut conn: &TcpStream) {
    /*
        Returns something that then gets sent to the connection
    */
    let data = database::get(&name);
    let parsed = serde_json::to_string(&data).unwrap();
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
        "BUY"   => todo!("BUYING"),
        "SELL"  => todo!("SELLING"),
        "QUERY" => query_stock(stock, &conn).await,
        _ => todo!("UNKNOWN COMMAND!!!"),
    };
}

async fn get_stocks(time_out: u64) {
    /*
    loop {
        get all stocks from database
        update their prices
        timeout for an hour
    }
    */
    
    // this will do for now
    loop {
        let names : Vec<String> = database::get_names();
        println!("Database stock names: {:?}", &names);
        for name in names {
            let stock_struct : Stock = yahoo::get_req(&name).await;
            database::insert_stock(stock_struct, "stocks.db").await;
        }
        println!("Hello!, sleeping for {} seconds...", time_out);
        tokio::time::sleep(time::Duration::from_secs(time_out)).await;
    }
}

#[tokio::main]
async fn main() {
    // Some basic stocks to pad out the DB
    // initialise().await;

    const time_out : u64 = 60 * 60;
    tokio::spawn(async move {
        get_stocks(time_out).await;
    });

    let listener = TcpListener::bind("127.0.0.1:7690")
        .expect("PORT DIDNT OPEN WHY GOT WHY");

    for stream in listener.incoming() {
        let mut curr_stream = stream.unwrap();
        tokio::spawn(async move {
            handle_conn(&mut curr_stream).await;
        });
    }

    /*
        loop forever {
            if a person requests a stock to either buy or sell {
                if the stock hasnt been updated in an hour {
                    update_stock()
                    put_into_db()
                    return updated price from db to user
                } else {
                    return current price from db
                }
            }
        }
    */
}