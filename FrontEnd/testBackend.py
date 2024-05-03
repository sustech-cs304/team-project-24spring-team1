import requests

url = "https://backend.sustech.me/api/auth/login"
payload = {
    "sustech_id": 12111611,
    "password": "jrxjrxjrx"
}
headers = {
    "Content-Type": "application/json"
}

response = requests.post(url, json=payload, headers=headers)

if response.status_code == 200:
    print("Success Message: ", response.text)
else:
    print("Error Code: ", response.status_code)
    print("Error Message: ", response.text)
