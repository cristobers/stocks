import discord
import extensions.stocks as stocks

def embed(type, name, mp, amount):
    embed = discord.Embed()
    embed.add_field(name="Price", value=mp, inline=False)
    if amount != None:
        embed.add_field(name="You currently have", value=amount, inline=False)
    return embed

def info_embed(lst, money):
    embed = discord.Embed()
    embed.add_field(name="monies", value=money)
    if len(lst) > 0:
        for entry in lst:
            name = entry[0][1]
            embed.add_field(
                name=f"{entry[0][1]} {entry[1]}", 
                value=stocks.get_price(name), 
                inline=False
            )
    else:
        embed.add_field(
                name="No stocks!", 
                value="You don't have any stocks, buy some with `/buy`", 
                inline=False
            )
    return embed
