# 🦀🐍 Telegram WebApp Auth Library
This library is a simple way to authenticate users in your web application using Telegram.

## Example

For a complete example, check out the [teleapp_auth repository](https://github.com/vffuunnyy/teleapp_auth).

## Installation

### Using pip

```sh
pip install teleapp-auth
```

### Using Poetry

```sh
poetry add teleapp-auth
```

### Using pipenv

```sh
pipenv install teleapp-auth
```

### Using uv

```sh
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
    auth_data = request_json.get("auth_data")  # Telegram auth data from the webapp
    webapp_data = parse_webapp_data(auth_data)  # Parse the auth data to dict

    # Example of the parsed data:
    # WebAppInitData {
    #   query_id: "AAEt6-JYAAAAAC3r4lj2oADQ",
    #   user: WebAppUser {
    #       id: 1491266349,
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