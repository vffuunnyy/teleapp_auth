from fastapi import Request, FastAPI
from teleapp_auth import get_secret_key, parse_webapp_data, validate_webapp_data


app = FastAPI()
secret_key = get_secret_key("BOT_TOKEN")


@app.post("/check_data")
async def check_data(request: Request) -> bool:
    request_json = await request.json()
    auth_data = request_json.get("auth_data")
    webapp_data = parse_webapp_data(auth_data)

    return validate_webapp_data(webapp_data, secret_key)
