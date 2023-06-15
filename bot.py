import telebot;
from telebot import types;
import time;

bot = telebot.TeleBot("6182954429:AAFhs3NF6Diynl2lFBzNvJGpwtTd9GGshMs")
admins = []
@bot.message_handler(commands=['start'])
def start(message):
  bot.send_message(message.chat.id, text="Приветствую вас, чего желаете?")
  
def Work():
   try:
     y = bot.polling(none_stop=True, interval=0.5)
     bot.polling(none_stop=True, interval=0.5)
     return y
   except:
      time.sleep(60)
      Work()


Work()
