import time
from random import randint
from selenium import webdriver
from selenium.webdriver.common.keys import Keys
from time import gmtime, strftime
d = False
#min = 5
min = randint(1, 15)
hor = 8
sec = 0
wait = 2


def login():
    driver = webdriver.Firefox()
    driver.implicitly_wait(10)
    driver.get("https://students.genesisedu.com/bernardsboe/")
    assert "Students" in driver.title
    username = driver.find_element_by_name('j_username')
    password = driver.find_element_by_name('j_password')
    username.clear()
    password.clear()
    # replace "<Email>" with your email (SamSmith@bernardsboe.com)
    username.send_keys("<Email>")
    password.send_keys("<Password>")  # replace <Password> with password
    password.send_keys(Keys.RETURN)
    button = driver.find_element_by_id('__button1__')
    button.click()
    element = driver.find_element_by_id('attendanceType')
    all_options = element.find_elements_by_tag_name("option")
    for option in all_options:
        if option.get_attribute("value") == "Present":
            option.click()
    save = driver.find_element_by_class_name('saveButton')
    save.click()
    time.sleep(3)
    driver.close()


while True:
    # change - 4 to your timezone (-4 is Eastern Time)
    h = int(strftime("%H", gmtime())) - 4
    m = int(strftime("%M", gmtime()))
    ct = m + (h * 60)
    ft = min + (hor * 60)
    tp = ft - ct
    print(chr(27) + "[2J")
    if tp < 0:
        pt = tp + 24 * 60
        ph = pt / 60
        pm = pt % 60
        print(str(int(ph)) + ":" + str(pm))
    else:
        pt = tp
        ph = pt / 60
        pm = pt % 60
        print(str(int(ph)) + ":" + str(pm))
    if int(h) == hor:
        if int(m) == min:
            if d == False:
                login()
                time.sleep(65)
    time.sleep(wait)
