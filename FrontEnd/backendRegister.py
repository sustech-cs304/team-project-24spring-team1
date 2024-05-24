import requests

url = "https://backend.sustech.me/api/auth/register"
payload = {
    "sustech_id": 12111602,
    "name": "JJJJJJ",
    "password": "jrxjrxjrx"
}
headers = {
    "Content-Type": "application/json"
}

response = requests.post(url, json=payload)

if response.status_code == 200:
    print("Success Message: ", response.text)
else:
    print("Error Code: ", response.status_code)
    print("Error Message: ", response.text)
