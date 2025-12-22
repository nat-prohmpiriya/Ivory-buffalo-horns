-- Add new columns for Firebase auth
ALTER TABLE users
    ALTER COLUMN email DROP NOT NULL,
    ADD COLUMN IF NOT EXISTS display_name VARCHAR(255),
    ADD COLUMN IF NOT EXISTS photo_url TEXT,
    ADD COLUMN IF NOT EXISTS provider VARCHAR(50) DEFAULT 'unknown',
    ADD COLUMN IF NOT EXISTS last_login_at TIMESTAMPTZ DEFAULT NOW();

-- Update existing users to have a provider
UPDATE users SET provider = 'unknown' WHERE provider IS NULL;
