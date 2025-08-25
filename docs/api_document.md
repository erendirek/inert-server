# Chat and Chat Application API Documentation (v1)

This document contains both REST API and WebSocket protocol definitions for a Discord-style chat application. This structure will be used to build a real-time messaging and server-based chat infrastructure.

## General

- **API Version:** v1
- **REST Base URL:** `https://domain/api/rest`
- **WebSocket URL:** `wss://domain/api/ws`

---

## REST API

The REST API is used for data management and history queries.

### Authentication

- [x] `POST /auth/register` — Creates a new user registration.
- [x] `POST /auth/login` — Performs the user login process.
- [ ] `POST /auth/logout` — Performs the user logout process.
- [ ] `POST /auth/refresh` — Gets a new access token with a refresh token.
- [x] `GET /auth/me` — Retrieves user information with a valid JWT token.

### Servers

- [x] `GET /servers` — Lists all servers the user is in.
- [x] `POST /servers` — Creates a new server.
- [x] `GET /servers/{server_id}` — Retrieves server details.
- [ ] `PATCH /servers/{server_id}` — Updates server settings.
- [ ] `DELETE /servers/{server_id}` — Deletes the server.

### Channels

- [x] `GET /servers/{server_id}/channels` — Lists all channels on the server.
- [x] `POST /servers/{server_id}/channels` — Creates a new channel.

### Messages

- [x] `GET /channels/{channel_id}/messages?limit=N&before=X` — Retrieves channel history messages (with paging support).
- [x] `POST /channels/{channel_id}/messages` — Sends a new message.
- [ ] `DELETE /channels/{channel_id}/messages/{message_id}` — Deletes a message.

### Users

- [x] `GET /users/{user_id}` — User profile information.
- [ ] `PATCH /users/{user_id}` — Profile update.

### DM Channels

- [ ] `GET /users/{user_id}/dm-channels` — Lists the user's DM channels.
- [ ] `POST /users/{user_id}/dm-channels` — Starts a new DM channel.

---

## WebSocket Protocol

WebSocket is used for the transmission of real-time events.

### Connection and Heartbeat

| OpCode | Event         | Description                         |
| ------ | ------------- | ----------------------------------- |
| 0      | HELLO         | Server reports the heartbeat interval |
| 1      | HEARTBEAT     | Client sends a heartbeat          |
| 2      | HEARTBEAT_ACK | Server acknowledges the heartbeat   |

### Events (Dispatch)

| OpCode | Event                | Description                                  |
| ------ | -------------------- | -------------------------------------------- |
| 10     | CONNECTED            | Connection successful, returns user information |
| 11     | MESSAGE_CREATE       | New message published                        |
| 12     | MESSAGE_UPDATE       | Message updated                            |
| 13     | MESSAGE_DELETE       | Message deleted                                |
| 20     | CHANNEL_CREATE       | New channel created                       |
| 21     | CHANNEL_UPDATE       | Channel information updated                    |
| 22     | CHANNEL_DELETE       | Channel deleted                                |
| 30     | SERVER_MEMBER_ADD    | User added to the server                   |
| 31     | SERVER_MEMBER_REMOVE | User removed from the server                |
| 40     | PRESENCE_UPDATE      | User status changed (online, idle, dnd) |
| 50     | TYPING_START         | User started typing                    |

---

### Sample WebSocket Messages

**MESSAGE_CREATE**

```json
{
  "op": 11,
  "d": {
    "message_id": "abc123",
    "channel_id": "chan456",
    "author": {
      "user_id": "u789",
      "username": "eren"
    },
    "content": "Hello!",
    "timestamp": "2025-05-11T12:34:56Z"
  }
}
```