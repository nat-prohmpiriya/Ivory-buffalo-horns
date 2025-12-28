-- Revert admin fields

-- Drop admin_logs table
DROP TABLE IF EXISTS admin_logs;

-- Drop indexes
DROP INDEX IF EXISTS idx_users_is_admin;

-- Remove columns from users
ALTER TABLE users DROP COLUMN IF EXISTS banned_reason;
ALTER TABLE users DROP COLUMN IF EXISTS banned_at;
ALTER TABLE users DROP COLUMN IF EXISTS is_admin;
