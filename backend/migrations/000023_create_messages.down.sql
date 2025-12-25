-- Reverse messages migration

DROP INDEX IF EXISTS idx_messages_conversation;
ALTER TABLE messages DROP COLUMN IF EXISTS conversation_id;

DROP TABLE IF EXISTS conversations;
DROP TABLE IF EXISTS message_reads;
DROP TABLE IF EXISTS messages;

DROP TYPE IF EXISTS message_type;
