import sqlite3

def cur_and_con(db_name):
    con = sqlite3.connect(db_name)
    return con.cursor(), con

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