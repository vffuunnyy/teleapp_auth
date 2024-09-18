from typing import Optional

def validate_webapp_data(webapp_data: dict[str, str], secret_key: bytes) -> bool:
    """
    Check if the bot is authorized to send messages to the user

    :param bot_token: Telegram bot token
    :param webapp_data: Webapp data

    :return: True if the bot is authorized to send messages to the user, False otherwise
    """

def get_secret_key(bot_token: str) -> bytes:
    """
    Get the secret key

    :param bot_token: Telegram bot token

    :return: The secret key
    """

def parse_webapp_data(webapp_data: str) -> WebAppInitData:
    """
    Parse the webapp data

    :param webapp_data: Webapp data

    :return: The parsed webapp data
    """

class WebAppUser:
    """
    This object contains the data of the Mini App user.

    Attributes:
        id (int): A unique identifier for the user or bot. It has at most 52 significant bits.
        is_bot (Optional[bool]): True if this user is a bot. Only returned in the receiver field.
        first_name (str): First name of the user or bot.
        last_name (Optional[str]): Optional. Last name of the user or bot.
        username (Optional[str]): Optional. Username of the user or bot.
        language_code (Optional[str]): Optional. IETF language tag of the user's language.
        is_premium (Optional[bool]): Optional. True if this user is a Telegram Premium user.
        added_to_attachment_menu (Optional[bool]): Optional. True if this user added the bot to the attachment menu.
        allows_write_to_pm (Optional[bool]): Optional. True if this user allowed the bot to message them.
        photo_url (Optional[str]): Optional. URL of the user’s profile photo. Can be in .jpeg or .svg format.
    """

    id: int
    is_bot: Optional[bool]
    first_name: str
    last_name: Optional[str]
    username: Optional[str]
    language_code: Optional[str]
    is_premium: Optional[bool]
    added_to_attachment_menu: Optional[bool]
    allows_write_to_pm: Optional[bool]
    photo_url: Optional[str]

class WebAppChat:
    """
    This object represents a chat.

    Attributes:
        id (int): Unique identifier for this chat. It has at most 52 significant bits.
        type_ (str): Type of chat. Can be either "group", "supergroup", or "channel".
        title (str): Title of the chat.
        username (Optional[str]): Optional. Username of the chat.
        photo_url (Optional[str]): Optional. URL of the chat's photo. Can be in .jpeg or .svg format.
    """

    id: int
    type: str  # Can be "group", "supergroup", or "channel"
    title: str
    username: Optional[str]
    photo_url: Optional[str]

class WebAppInitData:
    """
    This object contains data transferred to the Mini App when it is opened.

    Attributes:
        query_id (Optional[str]): Optional. A unique identifier for the Mini App session, required for sending messages via the answerWebAppQuery method.
        user (Optional[WebAppUser]): Optional. An object containing data about the current user.
        receiver (Optional[WebAppUser]): Optional. An object containing data about the chat partner of the current user, returned only for private chats when launched via the attachment menu.
        chat (Optional[WebAppChat]): Optional. An object containing data about the chat where the bot was launched via the attachment menu.
        chat_type (Optional[str]): Optional. Type of the chat from which the Mini App was opened. Can be "sender", "private", "group", "supergroup", or "channel".
        chat_instance (Optional[str]): Optional. Global identifier, uniquely corresponding to the chat from which the Mini App was opened.
        start_param (Optional[str]): Optional. Value of the startattach parameter, passed via link, returned for Mini Apps launched from the attachment menu.
        can_send_after (Optional[int]): Optional. Time in seconds after which a message can be sent via the answerWebAppQuery method.
        auth_date (Optional[int]): Unix time when the form was opened.
        hash (Optional[str]): A hash of all passed parameters, which the bot server can use to check their validity.
    """

    query_id: Optional[str]
    user: Optional[WebAppUser]
    receiver: Optional[WebAppUser]
    chat: Optional[WebAppChat]
    chat_type: Optional[
        str
    ]  # Can be "sender", "private", "group", "supergroup", or "channel"
    chat_instance: Optional[str]
    start_param: Optional[str]  # Value of the startattach parameter
    can_send_after: Optional[int]  # Time in seconds
    auth_date: Optional[int]  # Unix time
    hash: Optional[str]
