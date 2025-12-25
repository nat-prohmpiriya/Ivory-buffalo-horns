-- Messages System: Private messages between players and alliance messages

-- Message type enum
CREATE TYPE message_type AS ENUM ('private', 'alliance');

-- Messages table
CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Message type
    message_type message_type NOT NULL DEFAULT 'private',

    -- Sender (always required)
    sender_id UUID NOT NULL REFERENCES users(id),

    -- Recipient for private messages (NULL for alliance messages)
    recipient_id UUID REFERENCES users(id),

    -- Alliance for alliance messages (NULL for private messages)
    alliance_id UUID REFERENCES alliances(id) ON DELETE CASCADE,

    -- Message content
    subject VARCHAR(200) NOT NULL,
    body TEXT NOT NULL,

    -- Read status per recipient
    -- For private: single recipient
    -- For alliance: handled by message_reads table
    is_read BOOLEAN NOT NULL DEFAULT FALSE,

    -- Soft delete per user
    sender_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    recipient_deleted BOOLEAN NOT NULL DEFAULT FALSE,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Constraints
    CONSTRAINT private_message_has_recipient
        CHECK (message_type != 'private' OR recipient_id IS NOT NULL),
    CONSTRAINT alliance_message_has_alliance
        CHECK (message_type != 'alliance' OR alliance_id IS NOT NULL)
);

-- Index for listing inbox (private messages to user)
CREATE INDEX idx_messages_recipient ON messages(recipient_id, created_at DESC)
    WHERE message_type = 'private' AND recipient_deleted = FALSE;

-- Index for listing sent messages
CREATE INDEX idx_messages_sender ON messages(sender_id, created_at DESC)
    WHERE sender_deleted = FALSE;

-- Index for alliance messages
CREATE INDEX idx_messages_alliance ON messages(alliance_id, created_at DESC)
    WHERE message_type = 'alliance';

-- Index for unread count
CREATE INDEX idx_messages_unread ON messages(recipient_id, is_read)
    WHERE message_type = 'private' AND recipient_deleted = FALSE AND is_read = FALSE;

-- Alliance message read tracking (for alliance messages only)
CREATE TABLE message_reads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    message_id UUID NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id),
    read_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_message_read UNIQUE (message_id, user_id)
);

-- Index for checking if user read alliance message
CREATE INDEX idx_message_reads_user ON message_reads(user_id, message_id);

-- Conversations table for grouping private message threads
CREATE TABLE conversations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_1_id UUID NOT NULL REFERENCES users(id),
    user_2_id UUID NOT NULL REFERENCES users(id),

    -- Last message info for preview
    last_message_id UUID REFERENCES messages(id),
    last_message_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Per-user delete tracking
    user_1_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    user_2_deleted BOOLEAN NOT NULL DEFAULT FALSE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Ensure user_1_id < user_2_id to prevent duplicates
    CONSTRAINT unique_conversation UNIQUE (user_1_id, user_2_id),
    CONSTRAINT ordered_users CHECK (user_1_id < user_2_id)
);

-- Index for finding conversations by user
CREATE INDEX idx_conversations_user1 ON conversations(user_1_id, last_message_at DESC);
CREATE INDEX idx_conversations_user2 ON conversations(user_2_id, last_message_at DESC);

-- Add conversation reference to messages
ALTER TABLE messages ADD COLUMN conversation_id UUID REFERENCES conversations(id);

-- Index for messages in a conversation
CREATE INDEX idx_messages_conversation ON messages(conversation_id, created_at DESC)
    WHERE conversation_id IS NOT NULL;
