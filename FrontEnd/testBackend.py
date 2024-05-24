import requests

# url = "https://backend.sustech.me/api/auth/register"
url_places = "https://backend.sustech.me/api/metadata/places"
url_login = "https://sso.cra.ac.cn/realms/cra-service-realm/protocol/cas/login?service=https://backend.sustech.me/api/auth/callback?identifier=${identifier}"
url_create = "https://backend.sustech.me/api/event"

payload = {
    "sustech_id": 12110001,
    "name": "team1",
    "password": "jrxjrxjrx"
}
headers = {
    "Content-Type": "application/json"
}

activity = {
    "name": "Event Name",
    "kind": "show",
    "description": "Event Description",
    "venue_id": 1,
    "start_at": "2021-05-21T12:00:00",
    "end_at": "2021-05-21T14:00:00",
    "tickets": 100 ,
    "registration_deadline": "2021-05-21T11:00:00"
}

# response = requests.post(url_create, json= activity)
# response = requests.get(url_login)
response = requests.get(url_create)

if response.status_code == 200:
    print("Success Message: ", response.text)
else:
    print("Error Code: ", response.status_code)
    print("Error Message: ", response.text)