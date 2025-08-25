# Error Codes and Descriptions

## 🔐 Auth & Authentication (1000–1099)

| Code  | Key                  | Description                             |
|------|--------------------------|--------------------------------------|
| 1001 | AUTH_INVALID_CREDENTIALS | Invalid email or password              |
| 1002 | AUTH_USER_NOT_FOUND      | User not found                 |
| 1003 | AUTH_USER_ALREADY_EXISTS | User already registered              |
| 1004 | AUTH_UNAUTHORIZED        | Access attempt without login     |
| 1005 | AUTH_TOKEN_EXPIRED       | Token has expired                 |

## 👤 User Operations (2000–2099)

| Code  | Key              | Description                   |
|------|----------------------|----------------------------|
| 2001 | USER_NOT_FOUND       | Specified user does not exist   |
| 2002 | USER_ALREADY_FRIEND  | You are already friends         |
| 2003 | USER_BLOCKED         | User blocked      |

## 🧵 Channel Operations (3000–3099)

| Code  | Key               | Description                   |
|------|-----------------------|----------------------------|
| 3001 | CHANNEL_NOT_FOUND     | Channel not found           |
| 3002 | CHANNEL_ACCESS_DENIED | Channel access denied          |
| 3003 | CHANNEL_ALREADY_EXISTS| Channel with the same name already exists        |

## 💬 Messaging (4000–4099)

| Code  | Key              | Description                        |
|------|----------------------|---------------------------------|
| 4001 | MESSAGE_NOT_FOUND    | Message not found                       |
| 4002 | MESSAGE_SEND_FAILED  | Sending failed              |
| 4003 | MESSAGE_TOO_LONG     | Message exceeded character limit    |

## 🧑‍🤝‍🧑 Servers (Guilds) (5000–5099)

| Code  | Key               | Description                    |
|------|-----------------------|-----------------------------|
| 5001 | SERVER_NOT_FOUND      | Server not found           |
| 5002 | SERVER_ACCESS_DENIED  | Server access denied         |

## 🧱 General / System Errors (7000–7999)

| Code  | Key               | Description                           |
|------|-----------------------|------------------------------------|
| 6001 | VALIDATION_FAILED     | Invalid data entry              |
| 6002 | INTERNAL_SERVER_ERROR | Unexpected server error          |
| 6003 | RATE_LIMITED          | Too many requests sent         |

## Additional errors
- 7001 DATABASE_ERROR
- 7002 INVALID_JSON_TYPE
- 7003 INVALID_PATH