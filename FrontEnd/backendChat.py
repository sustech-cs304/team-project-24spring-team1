import requests

urlText = "https://backend.sustech.me/api/chat/2/message"
urlGetID = "https://backend.sustech.me/api/chat/get_id"
urlGetChat = "https://backend.sustech.me/api/chat"
urlGetMessage = "https://backend.sustech.me/api/chat/1/message"
urlGetMember = "https://backend.sustech.me/api/chat/1/member"
headers = {
    "Authorization": "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjIsInJvbGUiOiJzdHVkZW50IiwiaWF0IjoxNzE2NDU2NDAwLCJleHAiOjE3MTY1NDI4MDB9.AwPsptt17egt2IhFVkjhcCYO0VrxC8mQl3cJC5-bEMs",
    "Content-Type": "application/json"
}
headers2 = {
    "Authorization": "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjQsInJvbGUiOiJzdHVkZW50IiwiaWF0IjoxNzE2NDU5NDA1LCJleHAiOjE3MTY1NDU4MDV9.BtXQouL0bnT_Cwxhh4AE8HmYCWkJFPGldhzu6LVH3PA",
    "Content-Type": "application/json"
}
headers3 = {
    "Authorization": "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjUsInJvbGUiOiJzdHVkZW50IiwiaWF0IjoxNzE2NDY4ODQ2LCJleHAiOjE3MTY1NTUyNDZ9.KlKQ54-62INfgL28AE9qGf0mRVqQ8QzGZmB58FTl2Ac",
    "Content-Type": "application/json"
}

getID = {
    'with': 5
}
text = {
    "content": "Message 1 from user 5 to user 2"
}
getChat = {
    'page': 1
}
getMessage = {
    'page': 1
}
getMember = {
    'page': 1
}

# try:
#     response = requests.get(urlGetID, headers=headers, params=getID)
#     if response.status_code == 200:
#         print("请求成功:", response.json())
#     else:
#         print("请求失败，状态码:", response.status_code)
#         print("响应内容:", response.text)
# except requests.exceptions.RequestException as e:
#     print("请求出现异常:", e)

# try:
#     response = requests.post(urlText, headers=headers3, json=text)

#     if response.status_code == 200:
#         print("请求成功:", response.json())
#     else:
#         print("请求失败，状态码:", response.status_code)
#         print("响应内容:", response.text)
# except requests.exceptions.RequestException as e:
#     print("请求出现异常:", e)

# try:
#     response = requests.get(urlGetChat, headers=headers, params=getChat)

#     if response.status_code == 200:
#         print("请求成功:", response.json())
#     else:
#         print("请求失败，状态码:", response.status_code)
#         print("响应内容:", response.text)
# except requests.exceptions.RequestException as e:
#     print("请求出现异常:", e)

try:
    response = requests.get(urlGetMessage, headers=headers, params=getMessage)

    if response.status_code == 200:
        print("请求成功:", response.json())
    else:
        print("请求失败，状态码:", response.status_code)
        print("响应内容:", response.text)
except requests.exceptions.RequestException as e:
    print("请求出现异常:", e)

# try:
#     response = requests.get(urlGetMember, headers=headers, params=getMember)

#     if response.status_code == 200:
#         print("请求成功:", response.json())
#     else:
#         print("请求失败，状态码:", response.status_code)
#         print("响应内容:", response.text)
# except requests.exceptions.RequestException as e:
#     print("请求出现异常:", e)