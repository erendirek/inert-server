# Kullanıcılar
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

# Sunucular
CREATE TABLE servers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT now()
);

# Sunucu Üyeleri
CREATE TABLE server_members (
    server_id UUID NOT NULL REFERENCES servers(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    joined_at TIMESTAMPTZ DEFAULT now(),
    nickname TEXT,
    PRIMARY KEY (server_id, user_id)
);

# Roller
CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    server_id UUID NOT NULL REFERENCES servers(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    permissions JSON NOT NULL DEFAULT '{}',
    is_default BOOLEAN NOT NULL DEFAULT FALSE
);

# Üyelerin Rollerle İlişkisi
CREATE TABLE server_member_roles (
    server_id UUID NOT NULL,
    user_id UUID NOT NULL,
    role_id UUID NOT NULL,
    PRIMARY KEY (server_id, user_id, role_id),
    FOREIGN KEY (server_id, user_id) REFERENCES server_members(server_id, user_id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
);

# Kanal Tablosu
CREATE TABLE channels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    server_id UUID NOT NULL REFERENCES servers(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now()
);

# DM Kanalı
CREATE TABLE dm_channels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user1_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    user2_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT now(),
    CONSTRAINT user_pair_unique UNIQUE (user1_id, user2_id),
    CONSTRAINT user_order CHECK (user1_id < user2_id)
);

# Mesajlar
CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    channel_id UUID REFERENCES channels(id) ON DELETE CASCADE,
    dm_channel_id UUID REFERENCES dm_channels(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    edited_at TIMESTAMPTZ,
    deleted BOOLEAN DEFAULT FALSE,
    CONSTRAINT only_one_channel CHECK (
        (channel_id IS NOT NULL AND dm_channel_id IS NULL)
     OR (channel_id IS NULL AND dm_channel_id IS NOT NULL)
    )
);

# Refresh Tokenlar
CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now()
);