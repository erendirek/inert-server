+----------------+       +----------------+       +----------------+
|    users       |<------+   servers      +<------+ refresh_tokens |
|----------------|       |----------------|       |----------------|
| id (PK)        |       | id (PK)        |       | id (PK)        |
| username       |       | name           |       | user_id (FK)   |
| email          |       | owner_id (FK)  |       | token          |
| password_hash  |       | created_at     |       | expires_at     |
| created_at     |       +----------------+       | created_at     |
| updated_at     |                                 +----------------+
+----------------+

         ^                                         ^
         |                                         |
         |                                         |
         |                                         |
         |                                         |
         |   +--------------------+      +--------------------+
         |   |  server_members     |      |  dm_channels        |
         |   |--------------------|      |---------------------|
         +---+ server_id (FK)     |<-----+ user1_id (FK)       |
             | user_id (FK)       |<-----+ user2_id (FK)       |
             | joined_at          |      | created_at          |
             | nickname           |      +---------------------+
             +--------------------+

         ^              ^                ^
         |              |                |
         |              |                |
         |              |                |
         |              |                |
+-------------------+   |      +-------------------------+
|    server_member_roles |      |         roles          |
|-----------------------|      |-------------------------|
| server_id (FK)        |<-----+ id (PK)                 |
| user_id (FK)          |       | server_id (FK)         |
| role_id (FK)          +------>| name                   |
+-----------------------+       | permissions (JSON)     |
                                | is_default             |
                                +-------------------------+

        |
        v
+----------------+
|   channels     |
|----------------|
| id (PK)        |
| server_id (FK) |
| name           |
| created_at     |
+----------------+

        |
        v
+----------------+
|   messages     |
|----------------|
| id (PK)        |
| channel_id (*) |
| dm_channel_id(*)|
| author_id (FK) |
| content        |
| created_at     |
| edited_at      |
| deleted        |
+----------------+

(*) channel_id veya dm_channel_id yalnızca biri dolu olur.
