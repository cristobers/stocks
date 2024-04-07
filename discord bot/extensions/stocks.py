import discord, sqlite3, socket
from json import loads
from discord.ext import commands

host, port = '127.0.0.1', 7690

def format(data):
    name = data['name']
    market_price = data['market_price']
    market_day_high = data['market_day_high']
    market_day_low = data['market_day_low']
    return name, market_price, market_day_high, market_day_low

def cur_and_con(db_name):
    con = sqlite3.connect(db_name)
    return con.cursor(), con

def connect_to_stocks(stock: str):
    command = bytes(f"QUERY {stock}\r\n", encoding="utf8")
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((host, port))
        s.sendall(command)
        return s.recv(1024).decode()

@commands.hybrid_command(name="buy", description="Buys a stock")
async def buy(ctx, stock, amount):
    """
    if (user has enough money):
        take away price of stock from user
        add stock to stock_to_user db
    else:
        say("you dont have enough go away!")
    """
    data = loads(
        connect_to_stocks(stock)
    )
    name, mp, mh, ml = format(data)
    await ctx.send(f"{ctx.author.display_name} is buying {amount} {name} for {float(mp)*int(amount)}")

@commands.hybrid_command(name="query", description="querys a stock")
async def query(ctx, stock):

    # dictionary of values from the DB.
    data = loads(
        connect_to_stocks(stock)
    )

    # TODO: make this an embed instead.
    await ctx.send(f"{data['name']} is currently at ${data['market_price']}")

@commands.hybrid_command(name="sell", description="sells a stock")
async def sell(ctx, stock, amount):
    """
    if (user has stock in stock_to_user db):
        delete that stock entry
        add money to user db entry
    else:
        say("you cant get ye flask!")
    """
    data = loads(
        connect_to_stocks(stock)
    )
    name, mp, mh, ml = format(data)
    await ctx.send(f"{ctx.author.display_name} is selling {amount} {name} for {float(mp)*int(amount)}")

async def setup(bot):
    bot.add_command(buy)
    bot.add_command(sell)
    bot.add_command(query)