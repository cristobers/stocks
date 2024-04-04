use tokio;

mod yahoo;
mod database;

#[tokio::main]
async fn main() {
    
    database::create_stocks();
    let stocks: Vec<&str> = ["HAKSFHKJ", "GME", "NVDA", "AMD"].to_vec();

    for stock in stocks.into_iter() {
        let stock_info = yahoo::get_stock(stock).await;
        if stock_info.name != "None" {
            database::insert_stock(stock_info, "stocks.db").await;
        }
    }
}