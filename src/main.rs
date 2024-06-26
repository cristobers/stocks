use tokio;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::io::Write;
use std::io::Read;

mod yahoo;
mod database;

async fn initialise() {
    let stocks: Vec<&str> = [
        "GME",
        "NVDA",
        "AMD",
        "ITEL",
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
    let mut initial_query = database::get(&name);
    let database_get_failed = initial_query.name == "None";
    if database_get_failed || database::should_we_pull_new_prices(&initial_query) {
        println!("Getting new information for: {}", &name);
        // TODO: DONT RETURN NONE STOCK TO USER, THIS IS BAD!!!
        let updated_stock = yahoo::get_req(&name).await;
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

    conn.read(&mut buf)
        .unwrap();

    let stringed = String::from_utf8_lossy(&buf);
    let parse_input : Vec<&str> = stringed.split('\0')
        .collect();

    let full_cmd = parse_input[0];
    let split_cmd : Vec<&str> = full_cmd
        .split_whitespace()
        .collect();

    let (cmd, stock);
    match split_cmd.len() {
        2 => (cmd, stock) = (split_cmd[0], split_cmd[1]),
        _ => panic!("incorrect number of arguments"),
    }

    match cmd {
        "QUERY" => query_stock(stock, &conn).await,
        _ => todo!("UNKNOWN COMMAND!!!"),
    };
}

#[tokio::main]
async fn main() {
    if !Path::new("stocks.db").exists() {
        database::create_stocks();
        database::create_users();
        database::create_users_to_stocks();
        initialise().await;
    }

    let listener = TcpListener::bind("127.0.0.1:7690")
        .expect("Failed to listen on localhost:7690");

    for stream in listener.incoming() {
        let mut curr_stream = stream.unwrap();
        tokio::spawn(async move {
            handle_conn(&mut curr_stream).await;
        });
    }
}
