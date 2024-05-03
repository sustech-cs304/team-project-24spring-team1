import requests

url = "https://backend.sustech.me/api/account/2/profile"
payload = {
    "sustech_id": 12111611,
    "password": "jrxjrxjrx"
}
headers = {
    "Content-Type": "application/json"
}

response = requests.get(url, headers=headers)

if response.status_code == 200:
    print("Success Message: ", response.text)
else:
    print("Error Code: ", response.status_code)
    print("Error Message: ", response.text)
