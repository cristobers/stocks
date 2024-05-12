# stocks

Discord bot that allows users to buy and sell stocks

The application is split into two parts, one being the Rust backend that grabs prices,
the other being a discord bot that handles the user interaction.

The discord bot also handles adding and removing stocks, and also adding and removing money from users.


## How does this work

The Rust backend and the Discord bot both share a database, which houses the stocks 
that have been queried by users. These are kept until an hour has passed, then their 
value can be updated.

Stocks are given a timestamp, if its been more than an hour since the last update,
then the stock is updated from Yahoo Finance, this is done so that we mitigate the 
risk of being rate limited by Yahoo Finance.

### Stocks table

| name | price | day_high | day_low | last_get_request |
| ---- | ----- | --------- | ------- | --------------- |

### Users table

| user_id | money |
| ------- | ----- |

### Users_to_stocks table

| user_id | stock_name |
| ------- | ---------- |


## Discord bot commands
```
/buy TSLA 50
>>> Bob Buying 50 stocks in TSLA for $1.69

/sell TSLA 50
>>> Selling 50 stocks in TSLA, you have 0 left. You have made $84.50

/query $TSLA
>>> $TSLA is selling for $441.
```
