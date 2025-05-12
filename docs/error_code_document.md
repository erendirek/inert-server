# Hata Kodları ve Açıklamaları

## 🔐 Auth & Kimlik Doğrulama (1000–1099)

| Kod  | Anahtar                  | Açıklama                             |
|------|--------------------------|--------------------------------------|
| 1001 | AUTH_INVALID_CREDENTIALS | Email veya şifre hatalı              |
| 1002 | AUTH_USER_NOT_FOUND      | Kullanıcı bulunamadı                 |
| 1003 | AUTH_USER_ALREADY_EXISTS | Zaten kayıtlı kullanıcı              |
| 1004 | AUTH_UNAUTHORIZED        | Giriş yapılmadan erişim denemesi     |
| 1005 | AUTH_TOKEN_EXPIRED       | Tokenin suresi doldu                 |

## 👤 Kullanıcı İşlemleri (2000–2099)

| Kod  | Anahtar              | Açıklama                   |
|------|----------------------|----------------------------|
| 2001 | USER_NOT_FOUND       | Belirtilen kullanıcı yok   |
| 2002 | USER_ALREADY_FRIEND  | Zaten arkadaşsınız         |
| 2003 | USER_BLOCKED         | Kullanıcı engellenmiş      |

## 🧵 Kanal İşlemleri (3000–3099)

| Kod  | Anahtar               | Açıklama                   |
|------|-----------------------|----------------------------|
| 3001 | CHANNEL_NOT_FOUND     | Kanal bulunamadı           |
| 3002 | CHANNEL_ACCESS_DENIED | Kanal erişimi yok          |
| 3003 | CHANNEL_ALREADY_EXISTS| Aynı adda kanal var        |

## 💬 Mesajlaşma (4000–4099)

| Kod  | Anahtar              | Açıklama                        |
|------|----------------------|---------------------------------|
| 4001 | MESSAGE_NOT_FOUND    | Mesaj yok                       |
| 4002 | MESSAGE_SEND_FAILED  | Gönderme başarısız              |
| 4003 | MESSAGE_TOO_LONG     | Mesaj karakter sınırını aştı    |

## 🧑‍🤝‍🧑 Sunucular (Guilds) (5000–5099)

| Kod  | Anahtar               | Açıklama                    |
|------|-----------------------|-----------------------------|
| 5001 | SERVER_NOT_FOUND      | Sunucu bulunamadı           |
| 5002 | SERVER_ACCESS_DENIED  | Sunucuya erişim yok         |

## 🧱 Genel / Sistem Hataları (7000–7999)

| Kod  | Anahtar               | Açıklama                           |
|------|-----------------------|------------------------------------|
| 6001 | VALIDATION_FAILED     | Geçersiz veri girdisi              |
| 6002 | INTERNAL_SERVER_ERROR | Beklenmeyen sunucu hatası          |
| 6003 | RATE_LIMITED          | Çok fazla istek gönderildi         |

## Ek hatalar
7001 DATABASE_ERROR
7002 INVALID_JSON_TYPE