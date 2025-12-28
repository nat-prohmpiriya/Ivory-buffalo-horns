-- Add admin fields to users table

-- Add is_admin flag
ALTER TABLE users ADD COLUMN is_admin BOOLEAN NOT NULL DEFAULT FALSE;

-- Add banned_at for banning users
ALTER TABLE users ADD COLUMN banned_at TIMESTAMPTZ;
ALTER TABLE users ADD COLUMN banned_reason TEXT;

-- Create index for admin lookup
CREATE INDEX idx_users_is_admin ON users(is_admin) WHERE is_admin = TRUE;

-- Create admin_logs table for audit trail
CREATE TABLE admin_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    admin_id UUID NOT NULL REFERENCES users(id),
    action VARCHAR(50) NOT NULL,
    target_type VARCHAR(50) NOT NULL,  -- 'user', 'village', 'alliance', etc.
    target_id UUID,
    details JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_admin_logs_admin_id ON admin_logs(admin_id);
CREATE INDEX idx_admin_logs_target ON admin_logs(target_type, target_id);
CREATE INDEX idx_admin_logs_created_at ON admin_logs(created_at DESC);
