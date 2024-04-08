import discord, sqlite3, socket, os
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

def is_user_in_db(user_id):
    cur, _ = cur_and_con("../stocks.db")
    res = cur.execute("""
        SELECT user_id from users
        WHERE user_id = ?
    """, (user_id, ))
    res = res.fetchone()
    return res != None

def get_user_info(user_id):
    cur, _ = cur_and_con("../stocks.db")
    res = cur.execute("""
        SELECT * from users
        WHERE user_id = ?
    """, (user_id, ))
    res = res.fetchone()
    return res

def set_user_money(user_id, money):
    print(user_id, money)
    if not is_user_in_db(user_id):
        return
    cur, con = cur_and_con("../stocks.db")
    cur.execute("""
        UPDATE users
        set money = ?
        WHERE user_id = ?
    """, (money, user_id))
    con.commit()

def setup_user(user_id, starting_money):
    if is_user_in_db(user_id):
        return
    cur, con = cur_and_con("../stocks.db")
    cur.execute("""
        INSERT INTO users (user_id, money) 
        VALUES (?, ?)
    """, (user_id, starting_money))
    con.commit()
    
def give_user_stocks(user_id, stock_name, amount):
    cur, con = cur_and_con("../stocks.db")

    data = [(user_id, stock_name) for _ in range(amount)]

    cur.executemany("""
        INSERT INTO users_to_stocks (user_id, stock_name) 
        VALUES (?, ?)
    """, data)
    con.commit()

def count_user_stocks(user_id, stock_name) -> int:
    if not is_user_in_db(user_id):
        return
    cur, _ = cur_and_con("../stocks.db")
    res = cur.execute(
        """
        SELECT * FROM users_to_stocks
        WHERE stock_name = ?
        AND user_id = ?
        """, (stock_name, user_id))
    return len(res.fetchall())

def take_user_stocks(user_id, stock_name, amount) -> float:
    if not is_user_in_db(user_id):
        return
    cur, con = cur_and_con("../stocks.db")
    cur.execute(
        """
        DELETE FROM users_to_stocks
        WHERE user_id = ? 
        AND stock_name = ? 
        LIMIT ? 
        """, (user_id, stock_name, amount))
    con.commit()

def valid_input(n):
    if n <= 0:
        return False 
    return True

@commands.hybrid_command(name="buy", description="Buys a stock")
async def buy(ctx, stock, amount):
    user_id = ctx.author.id
    setup_user(user_id, 10_000)

    data = loads(connect_to_stocks(stock))
    _, user_money = get_user_info(user_id)

    name, mp, _, _ = format(data)
    user_money = '{0:.2f}'.format(user_money)
    mp, amount, user_money = float(mp), int(amount), float(user_money)

    if name == "None":
        await ctx.send("No stock found with this name.")
        return

    if user_money < (mp * amount) or user_money <= 0:
        await ctx.send(f"You cannot afford this. You only have `{user_money}`, this would cost you `{'{:.2f}'.format(mp*amount)}`")
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
    print(f"USER MONEY: {user_money} BUYING PRICE {buying_price} SUM {float(user_money)}")
    set_user_money(user_id, user_money)

    # now give the user the stocks they bought
    give_user_stocks(user_id, name, amount)
    await ctx.send(f"{ctx.author.display_name} is buying {amount} {name} for {buying_price}, you now have {user_money}")

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

    # TODO: make this an embed instead.
    await ctx.send(f"{data['name']} is currently at ${data['market_price']}")

@commands.hybrid_command(name="sell", description="sells a stock")
async def sell(ctx, stock, amount):
    user_id = ctx.author.id
    setup_user(user_id, 10_000)
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
    name, mp, _, _ = format(data)
    user_has = count_user_stocks(user_id, name)

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

    curr_user_amount = int(get_user_info(user_id)[1])

    final_amount = float(
        '{:.2f}'.format(mp * amount)
    )

    take_user_stocks(user_id, name, amount)
    set_user_money(user_id, curr_user_amount + final_amount)
    await ctx.send(f"{ctx.author.display_name} is selling {amount} {name} for {final_amount}")

async def setup(bot):
    bot.add_command(buy)
    bot.add_command(sell)
    bot.add_command(query)