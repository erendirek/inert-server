# Sohbet ve Chat Uygulaması API Dokümantasyonu (v1)

Bu belge, Discord tarzı bir sohbet uygulaması için hem REST API hem de WebSocket protokol tanımlarını içerir. Bu yapı, gerçek zamanlı mesajlaşma ve sunucu-temelli sohbet altyapısı kurmak için kullanılacaktır.

## Genel

- **API Versiyonu:** v1
- **REST Base URL:** `https://yourhost.com/api/rest`
- **WebSocket URL:** `wss://yourhost.com/api/ws`

---

## REST API

REST API, veri yönetimi ve geçmiş sorguları için kullanılır.

### Kimlik Dogrulama

- `POST /auth/register` — Yeni kullanıcı kaydı oluşturur.
- `POST /auth/login` — Kullanıcı giriş işlemini yapar.
- `POST /auth/logout` — Kullanıcı cikis işlemini yapar.
- `POST /auth/refresh` — Refresh token ile yeni access token alınır.
- `GET /auth/me` — Geçerli JWT token ile kullanıcı bilgilerini getirir.

### Sunucular

- `GET /servers` — Kullanıcının bulunduğu tüm sunucuları listeler
- `POST /servers` — Yeni sunucu oluşturur
- `GET /servers/{server_id}` — Sunucu detaylarını getirir
- `PATCH /servers/{server_id}` — Sunucu ayarlarını günceller
- `DELETE /servers/{server_id}` — Sunucuyu siler

### Kanallar

- `GET /servers/{server_id}/channels` — Sunucudaki tüm kanalları listeler
- `POST /servers/{server_id}/channels` — Yeni kanal oluşturur

### Mesajlar

- `GET /channels/{channel_id}/messages?limit=N&before=X` — Kanal geçmiş mesajlarını getirir (paging destekli)
- `POST /channels/{channel_id}/messages` — Yeni mesaj gönderir
- `DELETE /channels/{channel_id}/messages/{message_id}` — Mesaj siler

### Kullanıcılar

- `GET /users/{user_id}` — Kullanıcı profili bilgisi
- `PATCH /users/{user_id}` — Profil güncelleme

### DM Kanalları

- `GET /users/{user_id}/dm-channels` — Kullanıcının DM kanallarını listeler
- `POST /users/{user_id}/dm-channels` — Yeni DM kanalı başlatır

---

## WebSocket Protokolü

WebSocket, gerçek zamanlı olayların aktarımı için kullanılır.

### Bağlantı ve Heartbeat

| OpCode | Olay          | Açıklama                            |
| ------ | ------------- | ----------------------------------- |
| 0      | HELLO         | Sunucu heartbeat aralığını bildirir |
| 1      | HEARTBEAT     | İstemci heartbeat gönderir          |
| 2      | HEARTBEAT_ACK | Sunucu heartbeat'i onaylar          |

### Olaylar (Dispatch)

| OpCode | Event                | Açıklama                                     |
| ------ | -------------------- | -------------------------------------------- |
| 10     | CONNECTED            | Bağlantı başarılı, kullanıcı bilgisi döner   |
| 11     | MESSAGE_CREATE       | Yeni mesaj yayınlandı                        |
| 12     | MESSAGE_UPDATE       | Mesaj güncellendi                            |
| 13     | MESSAGE_DELETE       | Mesaj silindi                                |
| 20     | CHANNEL_CREATE       | Yeni kanal oluşturuldu                       |
| 21     | CHANNEL_UPDATE       | Kanal bilgisi güncellendi                    |
| 22     | CHANNEL_DELETE       | Kanal silindi                                |
| 30     | SERVER_MEMBER_ADD    | Sunucuya kullanıcı eklendi                   |
| 31     | SERVER_MEMBER_REMOVE | Sunucudan kullanıcı çıkarıldı                |
| 40     | PRESENCE_UPDATE      | Kullanıcı durumu değişti (online, idle, dnd) |
| 50     | TYPING_START         | Kullanıcı yazmaya başladı                    |

---

### Örnek WebSocket Mesajları

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
    "content": "Merhaba!",
    "timestamp": "2025-05-11T12:34:56Z"
  }
}
```
