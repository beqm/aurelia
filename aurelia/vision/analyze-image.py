import requests
import json


_key  = "key"
_url = "url"


path_to_file = "D:/Vault/Coding/Repositories/aurelia/data/display.png"
with open(path_to_file, 'rb') as f:
    data = f.read()


# Computer Vision parameters
params = {'mode' : 'Categories,Description,Color'}

headers = dict()
headers['Ocp-Apim-Subscription-Key'] = _key
headers['Content-Type'] = 'application/octet-stream'

json_d = None
response = requests.request( 'post', _url, json = json_d, data = data, headers = headers, params = params )
# Check if the request was successful
if response.status_code == 200:
    result = response.json()

    # Save the result to a JSON file
    output_file_path = "../data/result.json"
    with open(output_file_path, 'w', encoding='utf-8') as json_file:
        json.dump(result, json_file, ensure_ascii=False, indent=4)
    
    print(f"Result saved to {output_file_path}")
else:
    # Print error information if the request was not successful
    print(f"Error code: {response.status_code}")
    print(f"Message: {response.json()}")



