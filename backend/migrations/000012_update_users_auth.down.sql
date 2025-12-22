-- Revert changes
ALTER TABLE users
    DROP COLUMN IF EXISTS display_name,
    DROP COLUMN IF EXISTS photo_url,
    DROP COLUMN IF EXISTS provider,
    DROP COLUMN IF EXISTS last_login_at;

-- Note: Cannot revert email NOT NULL constraint if there are NULL values
