CREATE TYPE mission_type AS ENUM ('raid', 'attack', 'conquer', 'support', 'scout', 'settle');

CREATE TABLE armies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    player_id UUID NOT NULL REFERENCES players(id),
    from_village_id UUID NOT NULL REFERENCES villages(id),
    to_x INT NOT NULL,
    to_y INT NOT NULL,
    to_village_id UUID REFERENCES villages(id), -- NULL if empty tile (e.g. discard/adventure) or unknown yet
    mission mission_type NOT NULL,
    troops JSONB NOT NULL, -- {"infantry": 100, "war_elephant": 10}
    resources JSONB DEFAULT '{}', -- Carrying resources
    departed_at TIMESTAMPTZ NOT NULL,
    arrives_at TIMESTAMPTZ NOT NULL,
    returns_at TIMESTAMPTZ, -- NULL if one-way
    is_returning BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_armies_player ON armies(player_id);
CREATE INDEX idx_armies_arrives ON armies(arrives_at);
CREATE INDEX idx_armies_destination ON armies(to_x, to_y);
