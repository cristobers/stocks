import discord, socket, os, math
from collections import Counter
from json import load, loads
from discord.ext import commands
import extensions.db as db
import extensions.embeds as embeds

host, port = None, None
with open("extensions/settings.json") as f:
    settings = load(f)
    host, port = settings["HOST"], settings["PORT"]

def format(data):
    name = data['name']
    market_price = data['market_price']
    market_day_high = data['market_day_high']
    market_day_low = data['market_day_low']
    return name, market_price, market_day_high, market_day_low

def connect_to_stocks(stock: str):
    command = bytes(f"QUERY {stock}\r\n", encoding="utf8")
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((host, port))
        s.sendall(command)
        return s.recv(1024).decode()

def valid_input(n):
    if n <= 0:
        return False 
    return True

@commands.hybrid_command(name="buy", description="Buys a stock")
async def buy(ctx, stock, amount):
    user_id = ctx.author.id
    db.setup_user(user_id, 10_000)

    data = loads(connect_to_stocks(stock))
    _, user_money = db.get_user_info(user_id)

    name, mp, _, _ = format(data)
    user_money = '{0:.2f}'.format(user_money)
    mp, amount, user_money = float(mp), int(amount), float(user_money)

    if name == "None":
        await ctx.send("No stock found with this name.")
        return

    if user_money < (mp * amount) or user_money <= 0:
        # TODO: make this an embed
        await ctx.send(f"You cannot afford this. You only have `{user_money}`, this would cost you `{'{:.2f}'.format(mp*amount)}`. The max you can buy is {math.floor(user_money/mp)}.")
        return

    if not valid_input(amount):
        await ctx.send("Can't buy <= 0 shares")
        return

    for _ in range(amount):
        if user_money < mp:
            break
        user_money -= mp 

    if user_money <= 0:
        # i can forsee this maybe being an issue, so ill put this here just incase
        user_money = 0

    buying_price = float (
        '{:.2f}'.format(mp * int(amount))
    )

    user_money = float('{0:.2f}'.format(user_money))
    db.set_user_money(user_id, user_money)

    # now give the user the stocks they bought
    db.give_user_stocks(user_id, name, amount)
    stock_amount = db.count_user_stocks(user_id, name)
    await ctx.send(embed=embeds.embed("Buying", name, mp, stock_amount))

@commands.hybrid_command(name="query", description="querys a stock")
async def query(ctx, stock):

    # dictionary of values from the DB.
    data = loads(
        connect_to_stocks(stock)
    )

    name, _, _, _ = format(data)
    if name == "None":
        await ctx.send("No stock found with this name.")
        return

    name, mp, _, _ = format(data)
    await ctx.send(embed=embeds.embed("Querying", name, mp, None))

def get_price(stock_name: str):
    data = loads(connect_to_stocks(stock_name))
    _, mp, _, _ = format(data)
    return mp

@commands.hybrid_command(name="info", description="Gets your info")
async def info(ctx):
    db.setup_user(ctx.author.id, 10_000)
    res = db.get_stocks_for_user(ctx.author.id)
    final = Counter(res).most_common()
    _, money = db.get_user_info(ctx.author.id)
    await ctx.send(embed=embeds.info_embed(final, money))

@commands.hybrid_command(name="sell", description="sells a stock")
async def sell(ctx, stock, amount):
    user_id = ctx.author.id
    db.setup_user(user_id, 10_000)
    '''
    if (user has stock in stock_to_user db):
        delete that stock entry
        add money to user db entry
    else:
        say("you cant get ye flask!")
    '''
    data = loads(
        connect_to_stocks(stock)
    )
    name, mp, _, _ = format(data)
    user_has = db.count_user_stocks(user_id, name)

    if name == "None":
        await ctx.send("No stock found with this name.")
        return
    amount = int(amount)

    if not valid_input(amount):
        await ctx.send(f"You can't sell <= 0 shares.")
        return

    if user_has < amount:
        await ctx.send(f"You can't sell that many, you only have `{user_has}` shares")
        return

    curr_user_amount = int(db.get_user_info(user_id)[1])

    final_amount = float(
        '{:.2f}'.format(mp * amount)
    )

    db.take_user_stocks(user_id, name, amount)
    db.set_user_money(user_id, curr_user_amount + final_amount)
    user_has = db.count_user_stocks(user_id, name)
    await ctx.send(embed=embeds.embed("Selling", name, mp, user_has))

async def setup(bot):
    bot.add_command(buy)
    bot.add_command(sell)
    bot.add_command(query)
    bot.add_command(info)
