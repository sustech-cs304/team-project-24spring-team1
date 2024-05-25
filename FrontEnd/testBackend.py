import requests

# # url = "https://backend.sustech.me/api/auth/register"
# url_places = "https://backend.sustech.me/api/metadata/places"
# url_login = "https://sso.cra.ac.cn/realms/cra-service-realm/protocol/cas/login?service=https://backend.sustech.me/api/auth/callback?identifier=${identifier}"
# url_create = "https://backend.sustech.me/api/event"
#
# payload = {
#     "sustech_id": 12110001,
#     "name": "team1",
#     "password": "jrxjrxjrx"
# }
# headers = {
#     "Content-Type": "application/json"
# }
#
# activity = {
#     "name": "Event Name",
#     "kind": "show",
#     "description": "Event Description",
#     "venue_id": 1,
#     "start_at": "2021-05-21T12:00:00",
#     "end_at": "2021-05-21T14:00:00",
#     "tickets": 100 ,
#     "registration_deadline": "2021-05-21T11:00:00"
# }
#
# url_imgaes = url_create = "https://backend.sustech.me/upload"
#
# # response = requests.post(url_create, json= activity)
# # response = requests.get(url_login)
# response = requests.get(url_create)
#
# if response.status_code == 200:
#     print("Success Message: ", response.text)
# else:
#     print("Error Code: ", response.status_code)
#     print("Error Message: ", response.text)
#
# import requests
#
# # Define the login URL and credentials
# login_url = "https://backend.sustech.me/api/auth/login"
# login_data = {
#     "sustech_id": 12111611,
#     "password": "jrxjrxjrx"
# }
# # Perform login to get the session token
# session = requests.Session()
# login_response = session.post(login_url, json=login_data)  # Use json=login_data to send JSON payload
#
# print("Login request sent to:", login_url)
# print("Login request payload:", login_data)
#
# if login_response.status_code == 200:
#     token = login_response.json().get("token")  # Correctly extract the token from the JSON response
#     print("Token:", token)
#     print("Login successful")
# else:
#     print("Login failed")
#     print("Error Code:", login_response.status_code)
#     print("Error Message:", login_response.text)
#     exit()
#
#


# Define the event creation URL
event_url = "https://backend.sustech.me/api/event"

# Perform the GET request to list all events
params = {
    "kind": "show"  # Correctly using string keys for the dictionary
}
response_get_events = session.get(event_url, headers={"Authorization": f"Bearer {token}"})

if response_get_events.status_code == 200:
    print("Events retrieved successfully")
    print("Events: ", response_get_events.text)
else:
    print("Failed to retrieve events")
    print("Error Code: ", response_get_events.status_code)
    print("Error Message: ", response_get_events.text)
