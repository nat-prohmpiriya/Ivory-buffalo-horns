-- Alliances table
CREATE TABLE alliances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) NOT NULL UNIQUE,
    tag VARCHAR(4) NOT NULL UNIQUE,
    description TEXT,
    founder_id UUID NOT NULL REFERENCES users(id),
    leader_id UUID NOT NULL REFERENCES users(id),
    max_members INT NOT NULL DEFAULT 60,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Alliance member roles
CREATE TYPE alliance_role AS ENUM ('leader', 'officer', 'member');

-- Alliance members table
CREATE TABLE alliance_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    alliance_id UUID NOT NULL REFERENCES alliances(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role alliance_role NOT NULL DEFAULT 'member',
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(alliance_id, user_id),
    UNIQUE(user_id) -- A user can only be in one alliance
);

-- Invitation status
CREATE TYPE invitation_status AS ENUM ('pending', 'accepted', 'rejected', 'expired');

-- Alliance invitations table
CREATE TABLE alliance_invitations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    alliance_id UUID NOT NULL REFERENCES alliances(id) ON DELETE CASCADE,
    inviter_id UUID NOT NULL REFERENCES users(id),
    invitee_id UUID NOT NULL REFERENCES users(id),
    status invitation_status NOT NULL DEFAULT 'pending',
    message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '7 days',
    responded_at TIMESTAMPTZ
);

-- Diplomacy status
CREATE TYPE diplomacy_status AS ENUM ('neutral', 'ally', 'nap', 'enemy');

-- Alliance diplomacy table (relationships between alliances)
CREATE TABLE alliance_diplomacy (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    alliance_id UUID NOT NULL REFERENCES alliances(id) ON DELETE CASCADE,
    target_alliance_id UUID NOT NULL REFERENCES alliances(id) ON DELETE CASCADE,
    status diplomacy_status NOT NULL DEFAULT 'neutral',
    proposed_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(alliance_id, target_alliance_id)
);

-- Indexes for performance
CREATE INDEX idx_alliance_members_alliance_id ON alliance_members(alliance_id);
CREATE INDEX idx_alliance_members_user_id ON alliance_members(user_id);
CREATE INDEX idx_alliance_invitations_invitee_id ON alliance_invitations(invitee_id);
CREATE INDEX idx_alliance_invitations_status ON alliance_invitations(status);
CREATE INDEX idx_alliance_diplomacy_alliance_id ON alliance_diplomacy(alliance_id);
CREATE INDEX idx_alliance_diplomacy_target_id ON alliance_diplomacy(target_alliance_id);
