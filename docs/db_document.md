# Chat Application Database Design

This document presents a future-proof, extensible, and modular database design for a Discord-like chat application. The basic structures such as users, servers, channels, messages, roles, authentication, and DMs are detailed below.

---

## Tables

### 1. `users`

Stores user account information.

| Field         | Type      | Description             |
| ------------- | --------- | ----------------------- |
| id            | UUID (PK) | User ID                 |
| username      | TEXT      | Unique username         |
| email         | TEXT      | Email                   |
| password_hash | TEXT      | Password hash           |
| created_at    | TIMESTAMP | Registration date       |
| updated_at    | TIMESTAMP | Last update             |

---

### 2. `servers`

Stores servers created by users.

| Field      | Type      | Description        |
| ---------- | --------- | ------------------ |
| id         | UUID (PK) | Server ID          |
| name       | TEXT      | Server name        |
| owner_id   | UUID (FK) | Server owner       |
| created_at | TIMESTAMP | Creation date      |

---

### 3. `server_members`

Stores users who have joined servers.

| Field     | Type      | Description         |
| --------- | --------- | ------------------- |
| server_id | UUID (FK) | Server ID           |
| user_id   | UUID (FK) | User ID             |
| joined_at | TIMESTAMP | Join date           |
| nickname  | TEXT      | In-server nickname  |

**PK:** `(server_id, user_id)` (composite key)

---

### 4. `roles`

In-server roles.

| Field       | Type      | Description              |
| ----------- | --------- | ------------------------ |
| id          | UUID (PK) | Role ID                  |
| server_id   | UUID (FK) | Which server it belongs to |
| name        | TEXT      | Role name                |
| permissions | JSON      | Permissions              |
| is_default  | BOOLEAN   | Is it the default role?  |

---

### 5. `server_member_roles`

Specifies which user has which role.

| Field     | Type      | Description  |
| --------- | --------- | ------------ |
| server_id | UUID (FK) | Server ID    |
| user_id   | UUID (FK) | User ID      |
| role_id   | UUID (FK) | Role ID      |

**PK:** `(server_id, user_id, role_id)`

---

### 6. `channels`

Text channels within servers.

| Field      | Type      | Description        |
| ---------- | --------- | ------------------ |
| id         | UUID (PK) | Channel ID         |
| server_id  | UUID (FK) | Server ID          |
| name       | TEXT      | Channel name       |
| created_at | TIMESTAMP | Creation date      |

---

### 7. `messages`

Text messages (channel and DM).

| Field         | Type      | Description              |
| ------------- | --------- | ---------------------- |
| id            | UUID (PK) | Message ID               |
| channel_id    | UUID (FK) | Channel ID (nullable)    |
| dm_channel_id | UUID (FK) | DM Channel ID (nullable) |
| author_id     | UUID (FK) | Sender                 |
| content       | TEXT      | Message content          |
| created_at    | TIMESTAMP | Sent date              |
| edited_at     | TIMESTAMP | Last edited            |
| deleted       | BOOLEAN   | Is it deleted?           |

> Only one of the `channel_id` and `dm_channel_id` fields should be filled.

---

### 8. `dm_channels`

One-to-one messaging channel between users.

| Field      | Type      | Description        |
| ---------- | --------- | ------------------ |
| id         | UUID (PK) | DM Channel ID      |
| user1_id   | UUID (FK) | User 1             |
| user2_id   | UUID (FK) | User 2             |
| created_at | TIMESTAMP | Creation date      |

> Should be kept sorted such that `user1_id < user2_id` (to ensure uniqueness).

---

### 9. `refresh_tokens`

Stores the user's active refresh tokens (optional).

| Field      | Type      | Description         |
| ---------- | --------- | ------------------- |
| id         | UUID (PK) | Token ID            |
| user_id    | UUID (FK) | User                |
| token      | TEXT      | Refresh token       |
| expires_at | TIMESTAMP | Expiration date     |
| created_at | TIMESTAMP | Creation date       |

---

## Structures That Can Be Added in the Future

- `voice_channels`: Voice chat channels
- `attachments`: Adding media files to messages
- `presence_status`: User online/dnd/idle status
- `server_invites`: Invitation system
- `audit_logs`: Moderation and event history

---