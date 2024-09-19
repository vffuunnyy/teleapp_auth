# 🦀🐍 Telegram WebApp Auth Library
This library is a simple way to authenticate users in your web application using Telegram.

## Example

For a complete example, check out the [Example Bot repository](https://github.com/vffuunnyy/example_teleapp_auth).

## Installation

### Using pip

```bash
pip install teleapp-auth
```

### Using Poetry

```bash
poetry add teleapp-auth
```

### Using pipenv

```bash
pipenv install teleapp-auth
```

### Using uv

```bash
uv pip install teleapp-auth
```

## Usage

```python
from fastapi import Request, FastAPI
from teleapp_auth import get_secret_key, parse_webapp_data, validate_webapp_data


app = FastAPI()
secret_key = get_secret_key("BOT_TOKEN")


@app.post("/check_data")
async def check_data(request: Request) -> bool:
    request_json = await request.json()
    auth_data = request_json.get("auth_data")  # Telegram auth data from the request
    webapp_data = parse_webapp_data(auth_data)  # Parse the auth data to the WebAppInitData object

    # Example of the parsed data:
    # WebAppInitData {
    #   query_id: "AAEt6-JYAAAAAC3r4lj2oADQ",
    #   user: WebAppUser {
    #       id: 1,
    #       is_bot: None,
    #       first_name: "Test",
    #       last_name: "",
    #       username: "test",
    #       language_code: "ru",
    #       is_premium: None,
    #       added_to_attachment_menu: None,
    #       allows_write_to_pm: true,
    #       photo_url: None
    #   },
    #  receiver: None,
    #  chat: None,
    #  chat_type: None,
    #  chat_instance: None,
    #  start_param: None,
    #  can_send_after: None,
    #  auth_date: 1726572911,
    #  hash: "f87a5a37a5b487700a35cb1e3d2e92afa67e4b67066c9f1fa2c34986c2350b6e
    # }
    

    return validate_webapp_data(webapp_data, secret_key)  # True if the data is valid, False otherwise
```

## FAQ

### 1. What is the purpose of this library?
This library simplifies the process of authenticating users in your web application using Telegram WebApp authentication.

### 2. Can I get the user's profile photo with this library?
The `photo_url` field in the authentication data contains a URL to the user's profile photo. However, it is only available for Mini Apps launched from the attachment menu. Currently, integration with the attachment menu is [only accessible to major advertisers](https://core.telegram.org/bots/webapps#adding-bots-to-the-attachment-menu) on the Telegram Ad Platform. All bots can still test this feature in the test server environment by contacting Botfather on the test server.

### 3. Does this library automatically handle sending authentication data to my server?
No, you need to handle the process of sending the authentication data to your server. You can find an example of how to do this in the [page.html](https://github.com/vffuunnyy/teleapp_auth/blob/main/examples/page.html) file within the [examples](https://github.com/vffuunnyy/teleapp_auth/blob/main/examples) directory.

### 4. How do I validate the authentication data?
Use the `validate_webapp_data` function from the library to verify the authenticity of the data. This function compares the data with a secret key derived from your bot's token.

### 5. Can I use this library with any Python web framework?
Yes, the library is framework-agnostic, though [examples](https://github.com/vffuunnyy/teleapp_auth/blob/main/examples) are provided with [FastAPI](https://github.com/vffuunnyy/teleapp_auth/blob/main/examples/fastapi_server.py) and [Blacksheep](https://github.com/vffuunnyy/teleapp_auth/blob/main/examples/blacksheep_server.py). You can adapt it to other frameworks with minimal effort.