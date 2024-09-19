import dataclasses

from blacksheep import Application, Request
from teleapp_auth import get_secret_key, parse_webapp_data, validate_webapp_data


app = Application()
secret_key = get_secret_key("BOT_TOKEN")


@dataclasses.dataclass
class AuthResponse:
    status: bool
    user: dict


@app.router.post("/api/check_data")
async def check_data(request: Request) -> AuthResponse:
    request_json = await request.json()
    auth_data = request_json.get("auth_data")
    webapp_data = parse_webapp_data(auth_data)

    return AuthResponse(
        status=validate_webapp_data(webapp_data, secret_key),
        user=webapp_data.user.to_dict() if webapp_data.user else {},
    )
