CREATE TYPE alliance_role AS ENUM ('leader', 'co_leader', 'officer', 'diplomat', 'member');

CREATE TABLE alliances (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    server_id UUID NOT NULL REFERENCES servers(id),
    name VARCHAR(50) NOT NULL,
    tag VARCHAR(8) NOT NULL,
    description TEXT,
    leader_id UUID NOT NULL REFERENCES players(id),
    bank_gold INT DEFAULT 0,
    max_members INT DEFAULT 50,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(server_id, name),
    UNIQUE(server_id, tag)
);

CREATE TABLE alliance_members (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    alliance_id UUID NOT NULL REFERENCES alliances(id) ON DELETE CASCADE,
    player_id UUID NOT NULL REFERENCES players(id),
    role alliance_role DEFAULT 'member',
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(player_id) -- Player can only join one alliance
);

CREATE TABLE alliance_diplomacy (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    alliance_id UUID NOT NULL REFERENCES alliances(id) ON DELETE CASCADE,
    target_alliance_id UUID NOT NULL REFERENCES alliances(id) ON DELETE CASCADE,
    relation VARCHAR(20) NOT NULL, -- 'nap', 'war', 'ally'
    created_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    UNIQUE(alliance_id, target_alliance_id)
);
