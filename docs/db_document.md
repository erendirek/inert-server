# Sohbet Uygulaması Veritabanı Tasarımı

Bu belge, Discord benzeri bir sohbet uygulaması için geleceğe yönelik, genişletilebilir ve modüler bir veritabanı tasarımı sunar. Aşağıda kullanıcılar, sunucular, kanallar, mesajlar, roller, kimlik doğrulama ve DM gibi temel yapılar detaylandırılmıştır.

---

## Tablolar

### 1. `users`

Kullanıcı hesap bilgilerini tutar.

| Alan          | Tip       | Açıklama                |
| ------------- | --------- | ----------------------- |
| id            | UUID (PK) | Kullanıcı ID            |
| username      | TEXT      | Benzersiz kullanıcı adı |
| email         | TEXT      | E-posta                 |
| password_hash | TEXT      | Şifre hash'i            |
| created_at    | TIMESTAMP | Kayıt tarihi            |
| updated_at    | TIMESTAMP | Son güncelleme          |

---

### 2. `servers`

Kullanıcıların oluşturduğu sunucuları tutar.

| Alan       | Tip       | Açıklama           |
| ---------- | --------- | ------------------ |
| id         | UUID (PK) | Sunucu ID          |
| name       | TEXT      | Sunucu adı         |
| owner_id   | UUID (FK) | Sunucu sahibi      |
| created_at | TIMESTAMP | Oluşturulma tarihi |

---

### 3. `server_members`

Sunuculara katılmış kullanıcıları tutar.

| Alan      | Tip       | Açıklama            |
| --------- | --------- | ------------------- |
| server_id | UUID (FK) | Sunucu ID           |
| user_id   | UUID (FK) | Kullanıcı ID        |
| joined_at | TIMESTAMP | Katılma tarihi      |
| nickname  | TEXT      | Sunucu içi takma ad |

**PK:** `(server_id, user_id)` (bileşik anahtar)

---

### 4. `roles`

Sunucu içi roller.

| Alan        | Tip       | Açıklama           |
| ----------- | --------- | ------------------ |
| id          | UUID (PK) | Rol ID             |
| server_id   | UUID (FK) | Hangi sunucuya ait |
| name        | TEXT      | Rol adı            |
| permissions | JSON      | Yetkiler           |
| is_default  | BOOLEAN   | Varsayılan rol mü? |

---

### 5. `server_member_roles`

Hangi kullanıcının hangi rolde olduğunu belirtir.

| Alan      | Tip       | Açıklama     |
| --------- | --------- | ------------ |
| server_id | UUID (FK) | Sunucu ID    |
| user_id   | UUID (FK) | Kullanıcı ID |
| role_id   | UUID (FK) | Rol ID       |

**PK:** `(server_id, user_id, role_id)`

---

### 6. `channels`

Sunucular içerisindeki metin kanalları.

| Alan       | Tip       | Açıklama           |
| ---------- | --------- | ------------------ |
| id         | UUID (PK) | Kanal ID           |
| server_id  | UUID (FK) | Sunucu ID          |
| name       | TEXT      | Kanal adı          |
| created_at | TIMESTAMP | Oluşturulma tarihi |

---

### 7. `messages`

Metin mesajları (kanal ve DM).

| Alan          | Tip       | Açıklama               |
| ------------- | --------- | ---------------------- |
| id            | UUID (PK) | Mesaj ID               |
| channel_id    | UUID (FK) | Kanal ID (nullable)    |
| dm_channel_id | UUID (FK) | DM Kanal ID (nullable) |
| author_id     | UUID (FK) | Gönderen               |
| content       | TEXT      | Mesaj içeriği          |
| created_at    | TIMESTAMP | Gönderim tarihi        |
| edited_at     | TIMESTAMP | Son düzenlenme         |
| deleted       | BOOLEAN   | Silinmiş mi?           |

> `channel_id` ve `dm_channel_id` alanlarından sadece biri dolu olmalıdır.

---

### 8. `dm_channels`

Kullanıcılar arası bire bir mesajlaşma kanalı.

| Alan       | Tip       | Açıklama           |
| ---------- | --------- | ------------------ |
| id         | UUID (PK) | DM Kanal ID        |
| user1_id   | UUID (FK) | Kullanıcı 1        |
| user2_id   | UUID (FK) | Kullanıcı 2        |
| created_at | TIMESTAMP | Oluşturulma tarihi |

> `user1_id < user2_id` olacak şekilde sıralı tutulmalı (eşsizliği sağlamak için).

---

### 9. `refresh_tokens`

Kullanıcının aktif refresh token'larını saklar (isteğe bağlı).

| Alan       | Tip       | Açıklama           |
| ---------- | --------- | ------------------ |
| id         | UUID (PK) | Token ID           |
| user_id    | UUID (FK) | Kullanıcı          |
| token      | TEXT      | Refresh token      |
| expires_at | TIMESTAMP | Geçerlilik bitişi  |
| created_at | TIMESTAMP | Oluşturulma tarihi |

---

## İleride Eklenebilecek Yapılar

- `voice_channels`: Sesli sohbet kanalları
- `attachments`: Mesajlara medya dosyası ekleme
- `presence_status`: Kullanıcı online/dnd/idle durumu
- `server_invites`: Davet sistemi
- `audit_logs`: Moderasyon ve olay geçmişi

---
