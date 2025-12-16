# Technical Plan: Tusk & Horn

## Role: Senior System Architect / CTO
## Reference: Product Specification (.docs/01-spec.md)

---

## Tech Stack

| Layer | Technology | Rationale |
|-------|------------|-----------|
| Frontend | Svelte/SvelteKit | Reactive, lightweight, excellent performance |
| Backend | Rust (Axum) | Memory safety, high performance, zero-cost abstractions |
| Database | PostgreSQL + SQLx | ACID compliance, JSONB for flexible data, compile-time checked queries |
| Cache | Redis | Pub/Sub for real-time, session management |
| Real-time | WebSockets (tokio-tungstenite) | Bi-directional communication for live updates |
| Auth | Firebase Authentication | Managed auth, social login out-of-the-box |
| i18n | svelte-i18n (FE) + JSONB (BE) | Multi-language support |

---

## 1. System Architecture

### 1.1 High-Level Architecture

```
+-----------------------------------------------------------------------------+
|                              CLIENT LAYER                                    |
+-----------------------------------------------------------------------------+
|  +------------------+  +------------------+  +------------------+            |
|  |   Web Browser    |  |   Mobile Web     |  |  Future Native   |            |
|  |   (SvelteKit)    |  |   (Responsive)   |  |  App (Phase 3)   |            |
|  +--------+---------+  +--------+---------+  +--------+---------+            |
+-----------+--------------------+--------------------+------------------------+
            |                    |                    |
            +--------------------+--------------------+
                                 | HTTPS / WSS
+--------------------------------+---------------------------------------------+
|                         LOAD BALANCER (Nginx/Caddy)                          |
+--------------------------------+---------------------------------------------+
                                 |
+--------------------------------+---------------------------------------------+
|                           APPLICATION LAYER                                  |
+--------------------------------+---------------------------------------------+
|  +-----------------------------------------------------------------------+   |
|  |                    RUST AXUM SERVER                                   |   |
|  |  +--------------+ +--------------+ +--------------+ +--------------+  |   |
|  |  |   REST API   | |  WebSocket   | | Game Engine  | |    Auth      |  |   |
|  |  |   Handler    | |   Handler    | |   (Tick)     | | (Firebase)   |  |   |
|  |  +--------------+ +--------------+ +--------------+ +--------------+  |   |
|  |  +--------------+ +--------------+ +--------------+ +--------------+  |   |
|  |  |   Village    | |   Combat     | |  Alliance    | |   Market     |  |   |
|  |  |   Module     | |   Module     | |   Module     | |   Module     |  |   |
|  |  +--------------+ +--------------+ +--------------+ +--------------+  |   |
|  +-----------------------------------------------------------------------+   |
+--------------------------------+---------------------------------------------+
                                 |
+--------------------------------+---------------------------------------------+
|                            DATA LAYER                                        |
+--------------------------------+---------------------------------------------+
|  +------------------+  +------------------+  +------------------+            |
|  |    PostgreSQL    |  |      Redis       |  |   File Storage   |            |
|  |   (Primary DB)   |  |  (Cache/Pub-Sub) |  |    (S3/Minio)    |            |
|  |                  |  |                  |  |                  |            |
|  |  - Users         |  |  - Sessions      |  |  - Assets        |            |
|  |  - Villages      |  |  - Game State    |  |  - Uploads       |            |
|  |  - Armies        |  |  - Pub/Sub       |  |  - Skins         |            |
|  |  - Alliances     |  |  - Leaderboard   |  |                  |            |
|  |  - Transactions  |  |                  |  |                  |            |
|  +------------------+  +------------------+  +------------------+            |
+-----------------------------------------------------------------------------+
```

### 1.2 Game Tick System

```
+------------------------------------------------------------------+
|                      GAME TICK ENGINE                             |
|                    (Runs every 1 second)                          |
+------------------------------------------------------------------+
|                                                                   |
|  1. Resource Production Tick (Every 1 min)                        |
|     +-> Update all villages' resources                            |
|                                                                   |
|  2. Army Movement Tick (Every 1 sec)                              |
|     +-> Check arriving armies                                     |
|     +-> Trigger battle calculations                               |
|     +-> Send notifications                                        |
|                                                                   |
|  3. Construction Tick (Every 1 sec)                               |
|     +-> Check completed buildings                                 |
|     +-> Update village stats                                      |
|                                                                   |
|  4. Troop Training Tick (Every 1 sec)                             |
|     +-> Check completed troops                                    |
|     +-> Add to garrison                                           |
|                                                                   |
|  5. Starvation Tick (Every 5 min)                                 |
|     +-> Check negative crop villages                              |
|     +-> Kill troops proportionally                                |
|                                                                   |
+------------------------------------------------------------------+
```

### 1.3 Real-time Communication Flow

```
+------------+         +----------------+         +--------------+
|   Client   | --WSS-->| Rust Server    | <-----> |    Redis     |
|  (Svelte)  |         | (tokio tasks)  | Pub/Sub |              |
+------------+         +----------------+         +--------------+
      |                       |                         |
      |  1. Connect           |                         |
      |---------------------->|                         |
      |                       |  2. Subscribe channel   |
      |                       |------------------------>|
      |                       |                         |
      |                       |  3. Game event occurs   |
      |                       |<------------------------|
      |  4. Push update       |                         |
      |<----------------------|                         |
      |                       |                         |
```

---

## 2. Data Model / Schema

### 2.1 Entity Relationship Diagram

```
+-------------+       +-------------+       +-------------+
|    users    |       |   servers   |       |   tribes    |
+-------------+       +-------------+       +-------------+
| id (PK)     |       | id (PK)     |       | id (PK)     |
| email       |       | name        |       | code        |
| password    |       | region      |       | name_i18n   |
| provider    |       | status      |       | description |
| created_at  |       | started_at  |       | bonuses     |
+-------------+       | ended_at    |       +-------------+
       |              +-------------+              |
       |                    |                      |
       +----------+---------+                      |
                  |                                |
                  v                                |
          +-------------+                          |
          |   players   |<-------------------------+
          +-------------+
          | id (PK)     |
          | user_id(FK) |
          | server_id   |
          | tribe_id    |
          | name        |
          | gold        |
          | vip_until   |
          | joined_at   |
          +-------------+
                  |
          +-------+-------+
          v               v
   +-------------+ +-------------+
   |  villages   | |  alliances  |
   +-------------+ +-------------+
   | id (PK)     | | id (PK)     |
   | player_id   | | server_id   |
   | name        | | name        |
   | x, y        | | tag         |
   | is_capital  | | leader_id   |
   | population  | | description |
   | wood        | | bank_gold   |
   | clay        | | created_at  |
   | iron        | +-------------+
   | crop        |        |
   | warehouse   |        v
   | granary     | +-----------------+
   | loyalty     | | alliance_members|
   +-------------+ +-----------------+
          |        | alliance_id     |
          |        | player_id       |
          |        | role            |
          |        | joined_at       |
          |        +-----------------+
          |
    +-----+-----+-----------+-----------+
    v           v           v           v
+----------+ +--------+ +-----------+ +-----------+
| buildings| | troops | |  armies   | |  reports  |
+----------+ +--------+ +-----------+ +-----------+
| id (PK)  | | id(PK) | | id (PK)   | | id (PK)   |
| village  | | village| | player_id | | player_id |
| type     | | type   | | from_x,y  | | type      |
| level    | | count  | | to_x,y    | | data      |
| queue_at | |        | | mission   | | is_read   |
| done_at  | |        | | troops    | | created_at|
+----------+ +--------+ | depart_at | +-----------+
                        | arrive_at |
                        | return_at |
                        | resources |
                        +-----------+
```

### 2.2 PostgreSQL Schema (DDL)

```sql
-- Extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ENUM Types
CREATE TYPE tribe_code AS ENUM ('phasuttha', 'nava', 'kiri');
CREATE TYPE server_status AS ENUM ('preparing', 'running', 'ended');
CREATE TYPE building_type AS ENUM (
    'main_building', 'warehouse', 'granary', 'barracks', 'stable',
    'elephant_ground', 'workshop', 'academy', 'smithy', 'market',
    'embassy', 'palace', 'residence', 'wall', 'rally_point',
    'cranny', 'hero_mansion', 'tavern', 'duck_farm',
    'woodcutter', 'clay_pit', 'iron_mine', 'crop_field'
);
CREATE TYPE troop_type AS ENUM (
    -- Phasuttha
    'infantry', 'spearman', 'war_elephant', 'buffalo_wagon',
    -- Nava
    'kris_warrior', 'sea_diver', 'war_prahu', 'merchant_ship',
    -- Kiri
    'crossbowman', 'mountain_warrior', 'highland_pony', 'trap_maker',
    -- Special
    'swamp_dragon', 'locust_swarm', 'battle_duck', 'portuguese_musketeer'
);
CREATE TYPE mission_type AS ENUM ('raid', 'attack', 'conquer', 'support', 'scout', 'settle');
CREATE TYPE alliance_role AS ENUM ('leader', 'co_leader', 'officer', 'diplomat', 'member');
CREATE TYPE report_type AS ENUM ('battle', 'trade', 'scout', 'support', 'system');

-- Users (Account level)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255), -- NULL if OAuth
    oauth_provider VARCHAR(50), -- 'google', 'facebook'
    oauth_id VARCHAR(255),
    avatar_url TEXT,
    language VARCHAR(5) DEFAULT 'th',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(oauth_provider, oauth_id)
);

-- Servers (Game worlds)
CREATE TABLE servers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code VARCHAR(20) UNIQUE NOT NULL, -- 'TH-01', 'SEA-01'
    name VARCHAR(100) NOT NULL,
    region VARCHAR(50) NOT NULL,
    status server_status DEFAULT 'preparing',
    speed_multiplier DECIMAL(3,1) DEFAULT 1.0,
    map_size_x INT DEFAULT 200,
    map_size_y INT DEFAULT 200,
    started_at TIMESTAMPTZ,
    ends_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Tribes (Static reference)
CREATE TABLE tribes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code tribe_code UNIQUE NOT NULL,
    name_i18n JSONB NOT NULL, -- {"th": "Phasuttha", "en": "Phasuttha"}
    description_i18n JSONB NOT NULL,
    bonus_attack DECIMAL(4,2) DEFAULT 1.0,
    bonus_defense DECIMAL(4,2) DEFAULT 1.0,
    bonus_speed DECIMAL(4,2) DEFAULT 1.0,
    bonus_capacity DECIMAL(4,2) DEFAULT 1.0
);

-- Players (User in a specific server)
CREATE TABLE players (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    server_id UUID NOT NULL REFERENCES servers(id),
    tribe_id UUID NOT NULL REFERENCES tribes(id),
    name VARCHAR(50) NOT NULL,
    gold INT DEFAULT 0,
    silver INT DEFAULT 0,
    vip_level INT DEFAULT 0,
    vip_expires_at TIMESTAMPTZ,
    protection_until TIMESTAMPTZ, -- Beginner protection
    is_banned BOOLEAN DEFAULT FALSE,
    last_active_at TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, server_id),
    UNIQUE(server_id, name)
);

-- Villages
CREATE TABLE villages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    player_id UUID NOT NULL REFERENCES players(id),
    name VARCHAR(50) NOT NULL,
    x INT NOT NULL,
    y INT NOT NULL,
    is_capital BOOLEAN DEFAULT FALSE,
    population INT DEFAULT 0,
    -- Resources (stored as decimals for precise production)
    wood DECIMAL(15,2) DEFAULT 500,
    clay DECIMAL(15,2) DEFAULT 500,
    iron DECIMAL(15,2) DEFAULT 500,
    crop DECIMAL(15,2) DEFAULT 500,
    -- Production rates per hour
    wood_production DECIMAL(10,2) DEFAULT 50,
    clay_production DECIMAL(10,2) DEFAULT 50,
    iron_production DECIMAL(10,2) DEFAULT 50,
    crop_production DECIMAL(10,2) DEFAULT 50,
    crop_consumption DECIMAL(10,2) DEFAULT 0,
    -- Capacity
    warehouse_capacity INT DEFAULT 800,
    granary_capacity INT DEFAULT 800,
    -- Defense
    wall_level INT DEFAULT 0,
    loyalty INT DEFAULT 100,
    -- Timestamps
    resources_updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(player_id, x, y)
);

-- Create spatial index for map queries
CREATE INDEX idx_villages_coords ON villages(x, y);
CREATE INDEX idx_villages_player ON villages(player_id);

-- Buildings
CREATE TABLE buildings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    village_id UUID NOT NULL REFERENCES villages(id) ON DELETE CASCADE,
    building_type building_type NOT NULL,
    slot INT NOT NULL, -- Position in village (0-39)
    level INT DEFAULT 0,
    is_upgrading BOOLEAN DEFAULT FALSE,
    upgrade_started_at TIMESTAMPTZ,
    upgrade_ends_at TIMESTAMPTZ,
    UNIQUE(village_id, slot)
);

CREATE INDEX idx_buildings_village ON buildings(village_id);
CREATE INDEX idx_buildings_upgrading ON buildings(is_upgrading) WHERE is_upgrading = TRUE;

-- Building Queue (for VIP multi-queue)
CREATE TABLE building_queue (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    village_id UUID NOT NULL REFERENCES villages(id) ON DELETE CASCADE,
    building_id UUID NOT NULL REFERENCES buildings(id) ON DELETE CASCADE,
    target_level INT NOT NULL,
    position INT NOT NULL, -- Queue position
    wood_cost INT NOT NULL,
    clay_cost INT NOT NULL,
    iron_cost INT NOT NULL,
    crop_cost INT NOT NULL,
    duration_seconds INT NOT NULL,
    started_at TIMESTAMPTZ,
    ends_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Troop Definition (Static reference)
CREATE TABLE troop_definitions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    troop_type troop_type UNIQUE NOT NULL,
    tribe_id UUID REFERENCES tribes(id), -- NULL for special troops
    name_i18n JSONB NOT NULL,
    description_i18n JSONB NOT NULL,
    -- Base stats
    attack INT NOT NULL,
    defense_infantry INT NOT NULL,
    defense_cavalry INT NOT NULL,
    speed INT NOT NULL, -- Tiles per hour
    carry_capacity INT NOT NULL,
    crop_consumption INT NOT NULL,
    -- Training
    training_time_seconds INT NOT NULL,
    wood_cost INT NOT NULL,
    clay_cost INT NOT NULL,
    iron_cost INT NOT NULL,
    crop_cost INT NOT NULL,
    -- Requirements
    required_building building_type NOT NULL,
    required_building_level INT DEFAULT 1
);

-- Troops (in villages)
CREATE TABLE troops (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    village_id UUID NOT NULL REFERENCES villages(id) ON DELETE CASCADE,
    troop_type troop_type NOT NULL,
    count INT DEFAULT 0,
    in_training INT DEFAULT 0,
    UNIQUE(village_id, troop_type)
);

-- Troop Training Queue
CREATE TABLE troop_queue (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    village_id UUID NOT NULL REFERENCES villages(id) ON DELETE CASCADE,
    troop_type troop_type NOT NULL,
    count INT NOT NULL,
    each_duration_seconds INT NOT NULL,
    started_at TIMESTAMPTZ NOT NULL,
    ends_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Armies (Moving troops)
CREATE TABLE armies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    player_id UUID NOT NULL REFERENCES players(id),
    from_village_id UUID NOT NULL REFERENCES villages(id),
    to_x INT NOT NULL,
    to_y INT NOT NULL,
    to_village_id UUID REFERENCES villages(id), -- NULL if empty tile
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

-- Alliances
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

-- Alliance Members
CREATE TABLE alliance_members (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    alliance_id UUID NOT NULL REFERENCES alliances(id) ON DELETE CASCADE,
    player_id UUID NOT NULL REFERENCES players(id),
    role alliance_role DEFAULT 'member',
    joined_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(player_id)
);

-- Alliance Diplomacy (NAP, War)
CREATE TABLE alliance_diplomacy (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    alliance_id UUID NOT NULL REFERENCES alliances(id) ON DELETE CASCADE,
    target_alliance_id UUID NOT NULL REFERENCES alliances(id) ON DELETE CASCADE,
    relation VARCHAR(20) NOT NULL, -- 'nap', 'war', 'ally'
    created_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    UNIQUE(alliance_id, target_alliance_id)
);

-- Reports
CREATE TABLE reports (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    player_id UUID NOT NULL REFERENCES players(id),
    report_type report_type NOT NULL,
    title VARCHAR(200) NOT NULL,
    data JSONB NOT NULL, -- Battle details, trade info, etc.
    is_read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_reports_player ON reports(player_id, created_at DESC);
CREATE INDEX idx_reports_unread ON reports(player_id, is_read) WHERE is_read = FALSE;

-- Messages (Alliance chat & Private)
CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    sender_id UUID NOT NULL REFERENCES players(id),
    recipient_id UUID REFERENCES players(id), -- NULL if alliance message
    alliance_id UUID REFERENCES alliances(id), -- NULL if private message
    content TEXT NOT NULL,
    is_read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_messages_recipient ON messages(recipient_id, created_at DESC);
CREATE INDEX idx_messages_alliance ON messages(alliance_id, created_at DESC);

-- Transactions (Payment history)
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    player_id UUID REFERENCES players(id), -- NULL if account-level purchase
    amount_thb DECIMAL(10,2) NOT NULL,
    gold_amount INT NOT NULL,
    payment_method VARCHAR(50) NOT NULL,
    payment_provider VARCHAR(50) NOT NULL,
    external_id VARCHAR(255), -- Payment gateway reference
    status VARCHAR(20) DEFAULT 'pending', -- pending, completed, failed, refunded
    created_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

CREATE INDEX idx_transactions_user ON transactions(user_id, created_at DESC);
CREATE INDEX idx_transactions_status ON transactions(status) WHERE status = 'pending';

-- Map Tiles (for terrain data)
CREATE TABLE map_tiles (
    server_id UUID NOT NULL REFERENCES servers(id),
    x INT NOT NULL,
    y INT NOT NULL,
    terrain VARCHAR(20) NOT NULL, -- 'grassland', 'forest', 'mountain', 'water', 'swamp'
    oasis_type VARCHAR(20), -- 'wood', 'clay', 'iron', 'crop', NULL
    oasis_bonus DECIMAL(4,2),
    PRIMARY KEY (server_id, x, y)
);

-- Session Store (Optional - can use Redis instead)
CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    token_hash VARCHAR(255) NOT NULL,
    ip_address INET,
    user_agent TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_sessions_token ON sessions(token_hash);
CREATE INDEX idx_sessions_user ON sessions(user_id);
```

---

## 3. API Definition

### 3.1 Authentication APIs

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/auth/register` | Register with email/password |
| POST | `/api/v1/auth/login` | Login with email/password |
| POST | `/api/v1/auth/oauth/{provider}` | OAuth login (google/facebook) |
| POST | `/api/v1/auth/refresh` | Refresh JWT token |
| POST | `/api/v1/auth/logout` | Logout (invalidate token) |
### 3.1 Authentication APIs (Firebase)
 
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/auth/verify` | Verify Firebase ID Token & Create Session/User |
| POST | `/api/v1/auth/logout` | Logout (invalidate session) |
| GET | `/api/v1/auth/me` | Get current user info |

### 3.2 Server & Player APIs

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/servers` | List available servers |
| GET | `/api/v1/servers/{id}` | Get server details |
| POST | `/api/v1/servers/{id}/join` | Join a server (create player) |
| GET | `/api/v1/players/me` | Get current player info |
| PATCH | `/api/v1/players/me` | Update player settings |
| GET | `/api/v1/tribes` | List all tribes |

### 3.3 Village APIs

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/villages` | List player's villages |
| GET | `/api/v1/villages/{id}` | Get village details |
| PATCH | `/api/v1/villages/{id}` | Update village (rename) |
| GET | `/api/v1/villages/{id}/buildings` | List buildings |
| POST | `/api/v1/villages/{id}/buildings` | Start building upgrade |
| DELETE | `/api/v1/villages/{id}/buildings/queue/{id}` | Cancel building |
| GET | `/api/v1/villages/{id}/troops` | List troops in village |
| POST | `/api/v1/villages/{id}/troops` | Start troop training |
| DELETE | `/api/v1/villages/{id}/troops/queue/{id}` | Cancel training |

### 3.4 Army & Combat APIs

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/armies` | List player's armies |
| POST | `/api/v1/armies` | Send army (raid/attack/support) |
| DELETE | `/api/v1/armies/{id}` | Recall army (if not arrived) |
| POST | `/api/v1/armies/simulate` | Simulate battle result |

### 3.5 Map APIs

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/map` | Get map tiles (with viewport) |
| GET | `/api/v1/map/{x}/{y}` | Get tile details |
| GET | `/api/v1/map/search?name=` | Search player/village by name |

### 3.6 Alliance APIs

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/alliances` | List alliances |
| POST | `/api/v1/alliances` | Create alliance |
| GET | `/api/v1/alliances/{id}` | Get alliance details |
| PATCH | `/api/v1/alliances/{id}` | Update alliance |
| DELETE | `/api/v1/alliances/{id}` | Dissolve alliance |
| POST | `/api/v1/alliances/{id}/invite` | Invite player |
| POST | `/api/v1/alliances/{id}/kick` | Kick member |
| POST | `/api/v1/alliances/{id}/promote` | Change member role |
| POST | `/api/v1/alliances/{id}/diplomacy` | Set diplomacy (NAP/War) |
| GET | `/api/v1/alliances/{id}/messages` | Get alliance chat |
| POST | `/api/v1/alliances/{id}/messages` | Send alliance message |

### 3.7 Report & Message APIs

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/reports` | List reports |
| GET | `/api/v1/reports/{id}` | Get report details |
| PATCH | `/api/v1/reports/{id}` | Mark as read |
| DELETE | `/api/v1/reports/{id}` | Delete report |
| GET | `/api/v1/messages` | List private messages |
| POST | `/api/v1/messages` | Send private message |

### 3.8 Shop & Payment APIs

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/shop/items` | List shop items |
| POST | `/api/v1/shop/purchase` | Initiate purchase |
| POST | `/api/v1/shop/webhook/{provider}` | Payment webhook |
| GET | `/api/v1/shop/transactions` | Purchase history |
| POST | `/api/v1/shop/use-gold` | Instant complete (building/training) |
| POST | `/api/v1/shop/subscribe` | Subscribe to VIP |

### 3.9 WebSocket Events

```typescript
// Client -> Server
interface ClientMessage {
  type: 'subscribe' | 'unsubscribe' | 'ping';
  channel?: string; // 'village:{id}', 'alliance:{id}', 'map:{x}:{y}'
}

// Server -> Client
interface ServerMessage {
  type: 'resource_update' | 'building_complete' | 'troop_complete' |
        'army_arrived' | 'under_attack' | 'report_received' |
        'message_received' | 'alliance_update' | 'pong';
  data: any;
  timestamp: number;
}

// Event Examples
{
  type: 'resource_update',
  data: {
    village_id: 'uuid',
    wood: 1234.56,
    clay: 1234.56,
    iron: 1234.56,
    crop: 1234.56
  }
}

{
  type: 'under_attack',
  data: {
    village_id: 'uuid',
    attacker_name: 'EnemyPlayer',
    arrives_at: '2025-12-07T15:30:00Z',
    troop_count: 150 // Only if scouted
  }
}
```

---

## 4. Component Structure

### 4.1 Backend (Rust) Structure

```
backend/
|-- Cargo.toml                   # Dependencies (like package.json)
|-- Cargo.lock                   # Lock file
|-- .env                         # Environment variables
|-- src/
|   |-- main.rs                  # Entry point
|   |-- lib.rs                   # Library root (optional)
|   |-- config/
|   |   +-- mod.rs               # Configuration loading
|   |-- server/
|   |   |-- mod.rs               # HTTP server setup
|   |   |-- routes.rs            # Route definitions
|   |   +-- middleware/
|   |       |-- mod.rs
|   |       |-- auth.rs          # Firebase token verification
|   |       |-- cors.rs          # CORS handling
|   |       +-- logging.rs       # Request logging (tower-http)
|   |-- handlers/
|   |   |-- mod.rs
|   |   |-- auth.rs              # Auth handlers
|   |   |-- player.rs            # Player handlers
|   |   |-- village.rs           # Village handlers
|   |   |-- building.rs          # Building handlers
|   |   |-- troop.rs             # Troop handlers
|   |   |-- army.rs              # Army handlers
|   |   |-- map.rs               # Map handlers
|   |   |-- alliance.rs          # Alliance handlers
|   |   |-- report.rs            # Report handlers
|   |   |-- message.rs           # Message handlers
|   |   |-- shop.rs              # Shop handlers
|   |   +-- websocket.rs         # WebSocket handlers
|   |-- services/
|   |   |-- mod.rs
|   |   |-- firebase.rs          # Firebase Auth integration
|   |   |-- player.rs            # Player logic
|   |   |-- village.rs           # Village logic
|   |   |-- building.rs          # Building logic
|   |   |-- troop.rs             # Troop logic
|   |   |-- combat.rs            # Battle calculations
|   |   |-- army.rs              # Army movement
|   |   |-- alliance.rs          # Alliance logic
|   |   |-- shop.rs              # Payment logic
|   |   +-- notification.rs      # Push notifications
|   |-- repositories/
|   |   |-- mod.rs
|   |   |-- user.rs              # User data access
|   |   |-- player.rs            # Player data access
|   |   |-- village.rs           # Village data access
|   |   |-- building.rs          # Building data access
|   |   |-- troop.rs             # Troop data access
|   |   |-- army.rs              # Army data access
|   |   |-- alliance.rs          # Alliance data access
|   |   |-- report.rs            # Report data access
|   |   +-- transaction.rs       # Transaction data access
|   |-- models/
|   |   |-- mod.rs
|   |   |-- user.rs              # User model
|   |   |-- player.rs            # Player model
|   |   |-- village.rs           # Village model
|   |   |-- building.rs          # Building model
|   |   |-- troop.rs             # Troop model
|   |   |-- army.rs              # Army model
|   |   |-- alliance.rs          # Alliance model
|   |   |-- report.rs            # Report model
|   |   +-- dto/                 # Data Transfer Objects
|   |       |-- mod.rs
|   |       |-- request.rs       # Request DTOs
|   |       +-- response.rs      # Response DTOs
|   |-- game/
|   |   |-- mod.rs
|   |   |-- engine.rs            # Game tick engine
|   |   |-- resource.rs          # Resource production
|   |   |-- combat.rs            # Combat system
|   |   |-- building_data.rs     # Building definitions
|   |   |-- troop_data.rs        # Troop definitions
|   |   +-- formula.rs           # Game formulas
|   |-- realtime/
|   |   |-- mod.rs
|   |   |-- hub.rs               # WebSocket hub
|   |   |-- client.rs            # WebSocket client
|   |   +-- events.rs            # Event definitions
|   |-- db/
|   |   |-- mod.rs
|   |   |-- postgres.rs          # PostgreSQL connection (SQLx)
|   |   +-- redis.rs             # Redis connection
|   |-- firebase/
|   |   |-- mod.rs
|   |   +-- client.rs            # Firebase Admin SDK
|   +-- error/
|       +-- mod.rs               # Custom error types (thiserror)
|-- migrations/
|   |-- 000001_init.up.sql
|   |-- 000001_init.down.sql
|   +-- ...
|-- Makefile
|-- Dockerfile
+-- docker-compose.yml
```

### 4.2 Frontend (SvelteKit) Structure

#### 4.2.1 Page Architecture (4 Pages + Modals)

แนวทาง: ใช้ 4 หน้าหลัก และจัดการ features อื่นๆ ผ่าน Modal/Drawer เพื่อ UX ที่ลื่นไหลเหมือนเกมจริง

> **Note (MVP):** ใช้ Single Server ก่อน ไม่มี server selection
> - Login → ถ้ายังไม่มี Player → /onboarding
> - Login → ถ้ามี Player แล้ว → /game/village

```
┌─────────────────────────────────────────────────────────────────┐
│  LANDING PAGE (/)                                               │
│  - Hero section แนะนำเกม                                         │
│  - Features highlight                                           │
│  ├── Modal: Login (Email/Google/Facebook)                       │
│  ├── Modal: Register                                            │
│  └── Modal: Forgot Password                                     │
├─────────────────────────────────────────────────────────────────┤
│  ONBOARDING (/onboarding) - ผู้ใช้ใหม่เท่านั้น                    │
│  - Full-screen tribe selection                                  │
│  - Player name input                                            │
│  - Auto-create first village after completion                   │
│  ├── Step 1: เลือกเผ่า (Phasuttha, Nava, Kiri)                  │
│  ├── Step 2: ตั้งชื่อ Player                                     │
│  └── Step 3: (Future) Tutorial / Intro                          │
├─────────────────────────────────────────────────────────────────┤
│  MAP (/game/map)                                                │
│  - Interactive world map (200x200 grid)                         │
│  - Mini resource bar                                            │
│  - Army movement indicators                                     │
│  ├── Modal: Tile Detail (click on any tile)                     │
│  ├── Modal: Send Army (Raid/Attack/Support/Scout)               │
│  ├── Modal: Army List (traveling armies)                        │
│  ├── Modal: Search Player/Village                               │
│  ├── Drawer: Alliance (shared, same as Village)                 │
│  └── Drawer: Notifications                                      │
├─────────────────────────────────────────────────────────────────┤
│  VILLAGE (/game/village)                                        │
│  - Village visual (building slots)                              │
│  - Resource bar + production rates                              │
│  - Quick action buttons                                         │
│  ├── Modal: Building Detail + Upgrade                           │
│  ├── Modal: Troop Training                                      │
│  ├── Modal: Resource Fields Management                          │
│  ├── Modal: Reports List + Detail                               │
│  ├── Modal: Messages (Private)                                  │
│  ├── Modal: Shop - Gold/VIP Purchase (Phase 2)                  │
│  ├── Drawer: Building Queue                                     │
│  ├── Drawer: Troop Queue                                        │
│  └── Drawer: Alliance (Phase 2) - with tabs:                    │
│        ├── Tab: Overview (home, bank, activity)                 │
│        ├── Tab: Members (list, invite, kick, promote)           │
│        ├── Tab: Diplomacy (NAP, War, Ally)                      │
│        └── Tab: Chat (real-time messaging)                      │
└─────────────────────────────────────────────────────────────────┘
```

#### 4.2.2 Modal Management Strategy

```
URL State Sync (Deep Linking Support):
- /game/village?modal=building&id=barracks
- /game/map?modal=send-army&target=150,120
- Browser back button = close modal

Modal Stack:
- Support nested modals (e.g., Building → Confirm Upgrade → Success)
- Maximum 3 levels deep

Mobile Adaptation:
- Modal → Bottom Sheet (slide up)
- Drawer → Full Screen Slide
```

#### 4.2.3 Directory Structure

```
frontend/
|-- src/
|   |-- routes/
|   |   |-- +layout.svelte           # Root layout (global providers)
|   |   |-- +page.svelte             # Landing page
|   |   |-- onboarding/
|   |   |   +-- +page.svelte         # Tribe selection + Player name (new users)
|   |   +-- game/
|   |       |-- +layout.svelte       # Game layout (auth guard, websocket)
|   |       |-- map/
|   |       |   +-- +page.svelte     # World map
|   |       +-- village/
|   |           +-- +page.svelte     # Village management
|   |-- lib/
|   |   |-- components/
|   |   |   |-- ui/                  # Base UI components
|   |   |   |   |-- Button.svelte
|   |   |   |   |-- Card.svelte
|   |   |   |   |-- Modal.svelte     # Base modal wrapper
|   |   |   |   |-- Drawer.svelte    # Slide drawer
|   |   |   |   |-- BottomSheet.svelte
|   |   |   |   |-- Toast.svelte
|   |   |   |   |-- Timer.svelte     # Countdown component
|   |   |   |   |-- ProgressBar.svelte
|   |   |   |   +-- Tooltip.svelte
|   |   |   |-- modals/              # All modal contents
|   |   |   |   |-- auth/
|   |   |   |   |   |-- LoginModal.svelte
|   |   |   |   |   +-- RegisterModal.svelte
|   |   |   |   |-- shared/
|   |   |   |   |   |-- PlayerProfileModal.svelte
|   |   |   |   |   +-- SettingsModal.svelte
|   |   |   |   |-- map/
|   |   |   |   |   |-- TileDetailModal.svelte
|   |   |   |   |   |-- SendArmyModal.svelte
|   |   |   |   |   |-- ArmyListModal.svelte
|   |   |   |   |   +-- SearchModal.svelte
|   |   |   |   +-- village/
|   |   |   |       |-- BuildingDetailModal.svelte
|   |   |   |       |-- TroopTrainingModal.svelte
|   |   |   |       |-- ResourceFieldsModal.svelte
|   |   |   |       |-- ReportsModal.svelte
|   |   |   |       |-- MessagesModal.svelte
|   |   |   |       +-- ShopModal.svelte          # Phase 2
|   |   |   |-- game/                # Game-specific components
|   |   |   |   |-- ResourceBar.svelte
|   |   |   |   |-- BuildingSlot.svelte
|   |   |   |   |-- BuildingGrid.svelte
|   |   |   |   |-- TroopCard.svelte
|   |   |   |   |-- MapCanvas.svelte    # Interactive map
|   |   |   |   |-- MapTile.svelte
|   |   |   |   |-- ArmyMarker.svelte
|   |   |   |   |-- VillageMarker.svelte
|   |   |   |   +-- QueueItem.svelte
|   |   |   |-- drawers/             # Drawer contents
|   |   |   |   |-- AllianceDrawer.svelte     # Tabs: Overview, Members, Diplomacy, Chat
|   |   |   |   |-- NotificationsDrawer.svelte
|   |   |   |   |-- BuildingQueueDrawer.svelte
|   |   |   |   +-- TroopQueueDrawer.svelte
|   |   |   +-- layout/
|   |   |       |-- Navbar.svelte        # Top bar (resources, nav)
|   |   |       |-- GameHeader.svelte    # In-game header
|   |   |       +-- MobileNav.svelte     # Bottom nav for mobile
|   |   |-- stores/
|   |   |   |-- auth.ts              # Auth state (Firebase)
|   |   |   |-- player.ts            # Player data
|   |   |   |-- village.ts           # Selected village state
|   |   |   |-- resources.ts         # Live resources (real-time update)
|   |   |   |-- armies.ts            # Active armies
|   |   |   |-- modal.ts             # Modal state management
|   |   |   |-- notifications.ts     # Toast/alerts
|   |   |   +-- websocket.ts         # WebSocket connection state
|   |   |-- services/
|   |   |   |-- api.ts               # REST API client
|   |   |   |-- websocket.ts         # WebSocket client
|   |   |   |-- firebase.ts          # Firebase Auth client
|   |   |   +-- storage.ts           # LocalStorage helpers
|   |   |-- utils/
|   |   |   |-- format.ts            # Number/date formatting
|   |   |   |-- timer.ts             # Countdown utilities
|   |   |   |-- map.ts               # Map coordinate helpers
|   |   |   +-- constants.ts         # Game constants
|   |   +-- i18n/
|   |       |-- index.ts             # i18n setup (svelte-i18n)
|   |       +-- locales/
|   |           |-- th.json
|   |           +-- en.json
|   |-- app.html
|   |-- app.css                      # Global styles (Tailwind)
|   +-- hooks.server.ts              # Server hooks (auth check)
|-- static/
|   |-- images/
|   |   |-- tribes/                  # Tribe artwork
|   |   |-- buildings/               # Building icons/sprites
|   |   |-- troops/                  # Troop icons/sprites
|   |   |-- map/                     # Map tiles/terrain
|   |   +-- ui/                      # UI elements
|   +-- favicon.ico
|-- svelte.config.js
|-- tailwind.config.js
|-- vite.config.js
|-- package.json
+-- tsconfig.json
```

---

## 5. Third-party Integrations

### 5.1 Core Dependencies

| Category | Library/Service | Purpose |
|----------|-----------------|---------|
| **Backend (Rust)** | | |
| HTTP Framework | axum | Fast, ergonomic web framework |
| Database | sqlx | Async PostgreSQL with compile-time checked queries |
| Cache | redis-rs | Redis client |
| WebSocket | tokio-tungstenite | WebSocket handling |
| Auth | firebase-admin-rs | Firebase Admin SDK (or custom JWT verification) |
| Validation | validator | Request validation with derive macros |
| Migration | sqlx-cli | DB migrations |
| Logging | tracing + tracing-subscriber | Structured logging |
| Config | config-rs | Configuration management |
| Serialization | serde + serde_json | JSON serialization |
| Async Runtime | tokio | Async runtime |
| Error Handling | thiserror + anyhow | Custom error types |
| **Frontend** | | |
| UI Framework | SvelteKit | Full-stack framework |
| Styling | Tailwind CSS | Utility-first CSS |
| i18n | svelte-i18n | Internationalization |
| State | Svelte stores | Reactive state |
| HTTP | fetch / ky | API requests |
| Date | date-fns | Date formatting |

### 5.2 External Services

| Service | Provider | Purpose |
|---------|----------|---------|
| **Authentication** | | |
| OAuth | Google, Facebook | Social login |
| **Payments** | | |
| Primary | Omise / 2C2P | Thai payment gateway |
| Alternative | Stripe | International payments |
| **Infrastructure** | | |
| Hosting | DigitalOcean / AWS | Cloud hosting |
| CDN | Cloudflare | Asset delivery, DDoS protection |
| Email | SendGrid / AWS SES | Transactional emails |
| Push | Firebase Cloud Messaging | Mobile push (Phase 3) |
| **Monitoring** | | |
| APM | Sentry | Error tracking |
| Metrics | Prometheus + Grafana | Performance monitoring |
| Logs | Loki / ELK | Log aggregation |

### 5.3 Development Tools

| Tool | Purpose |
|------|---------|
| Docker | Containerization |
| Docker Compose | Local development |
| GitHub Actions | CI/CD |
| clippy | Rust linting |
| rustfmt | Rust formatting |
| cargo-watch | Auto-reload on file changes |
| ESLint + Prettier | Frontend linting |
| Playwright | E2E testing |

---

## 6. Security & Scalability

### 6.1 Security Measures

#### Authentication & Authorization

```
+------------------------------------------------------------------+
|                    SECURITY LAYERS                                |
+------------------------------------------------------------------+
|  1. HTTPS/TLS                                                     |
|     +-> All traffic encrypted (Let's Encrypt / Cloudflare)        |
|                                                                   |
|  2. JWT Authentication                                            |
|     +-> Short-lived access tokens (15 min)                        |
|     +-> Long-lived refresh tokens (7 days)                        |
|  2. Firebase Authentication                                       |
|     +-> Client logs in via Firebase SDK (Email/Google/Facebook)   |
|     +-> Sends ID Token to Backend in Authorization header         |
|     +-> Backend verifies ID Token via Firebase Admin SDK          |
|                                                                   |
|  3. Rate Limiting                                                 |
|     +-> API: 100 req/min per IP                                   |
|     +-> Auth: 5 attempts per 15 min                               |
|     +-> WebSocket: 10 messages/sec                                |
|                                                                   |
|  4. Input Validation                                              |
|     +-> Strict schema validation (JSON Schema)                    |
|     +-> SQL injection prevention (parameterized queries)          |
|     +-> XSS prevention (output encoding)                          |
|                                                                   |
|  5. CORS Policy                                                   |
|     +-> Whitelist allowed origins                                 |
|     +-> Credentials only with explicit origins                    |
+------------------------------------------------------------------+
```

#### Game-Specific Security

- **Anti-Cheat**
  - All calculations server-side
  - Client only renders, never decides
  - Movement speed validation
  - Resource overflow detection

- **Multi-Account Prevention**
  - Device fingerprinting
  - IP address tracking
  - Suspicious pattern detection
  - Resource transfer limits for new accounts

- **Data Protection**
  - Password hashing (bcrypt/argon2)
  - PII encryption at rest
  - GDPR compliance (data export/deletion)

### 6.2 Scalability Strategy

#### Phase 1: Single Server (MVP)

```
+------------------------------------------+
|           Single VPS Setup                |
+------------------------------------------+
|  Nginx (Load Balancer)                    |
|    |                                      |
|    +-> Rust Server (8 CPU, 16GB RAM)      |
|          |                                |
|          +-> PostgreSQL (Single)          |
|          +-> Redis (Single)               |
|                                           |
|  Capacity: ~10,000 concurrent users       |
+------------------------------------------+
```

#### Phase 2: Horizontal Scaling

```
+------------------------------------------------------------------+
|                 Scaled Architecture                               |
+------------------------------------------------------------------+
|                                                                   |
|  +-------------+                                                  |
|  |  Cloudflare |  (CDN + DDoS Protection)                         |
|  +------+------+                                                  |
|         |                                                         |
|  +------+------+                                                  |
|  |Load Balancer|  (Nginx / HAProxy)                               |
|  +------+------+                                                  |
|         |                                                         |
|  +------+------+------+------+------+                             |
|  | Rust App 1 | Rust App 2 | Rust App 3 |  (Stateless)            |
|  +------+------+------+------+------+------+                      |
|         |             |             |                             |
|  +------+-------------+-------------+------+                      |
|  |              Redis Cluster              |  (Session)           |
|  +------------------+----------------------+                      |
|                     |                                             |
|  +------------------+----------------------+                      |
|  |         PostgreSQL Primary              |                      |
|  |              + Replicas                 |                      |
|  +-----------------------------------------+                      |
|                                                                   |
|  Capacity: ~50,000 concurrent users                               |
+------------------------------------------------------------------+
```

#### Database Optimization

- **Indexing Strategy**
  - Composite indexes for common queries
  - Partial indexes for active data
  - GIN indexes for JSONB columns

- **Query Optimization**
  - Prepared statements
  - Connection pooling (pgbouncer)
  - Read replicas for reports/analytics

- **Caching Strategy**
  ```
  Cache Hierarchy:
  1. Application Memory (moka/cached) - Hot data, 1-5 sec TTL
  2. Redis - Shared state, 5-60 sec TTL
  3. PostgreSQL - Source of truth
  ```

#### Game Tick Optimization

- Batch processing (update 1000 villages per tick)
- Separate game engine tokio task
- Event sourcing for army movements
- Eventual consistency for non-critical updates

### 6.3 Monitoring & Alerts

```yaml
# Key Metrics to Monitor
Performance:
  - API response time (P50, P95, P99)
  - Database query time
  - WebSocket message latency
  - Game tick duration

Availability:
  - Server uptime
  - Error rate (5xx)
  - Failed login rate

Business:
  - Active players (DAU/MAU)
  - Concurrent connections
  - Purchases per hour
  - New registrations

Alerts:
  - Error rate > 1%: Warning
  - Error rate > 5%: Critical
  - Response time P95 > 500ms: Warning
  - Response time P95 > 1s: Critical
  - CPU > 80%: Warning
  - Memory > 85%: Warning
  - Disk > 90%: Critical
```

---

## 7. Deployment Strategy

### 7.1 Development Environment

```bash
# Local development with Docker Compose
docker-compose up -d

# Services:
# - Rust server: localhost:8080
# - SvelteKit: localhost:5173
# - PostgreSQL: localhost:5432
# - Redis: localhost:6379
# - Adminer (DB UI): localhost:8081
```

### 7.2 CI/CD Pipeline

```yaml
# GitHub Actions Workflow
name: Deploy

on:
  push:
    branches: [main, develop]

jobs:
  test:
    - Run Rust tests (cargo test)
    - Run frontend tests
    - Run linters (clippy, rustfmt)

  build:
    - Build Rust binary (cargo build --release)
    - Build SvelteKit
    - Build Docker images

  deploy-staging:
    - Deploy to staging (on develop branch)
    - Run E2E tests

  deploy-production:
    - Deploy to production (on main branch)
    - Blue-green deployment
    - Rollback on failure
```

### 7.3 Infrastructure (Recommended)

| Environment | Provider | Specs | Monthly Cost (Est.) |
|-------------|----------|-------|---------------------|
| **Development** | Local | Docker Compose | $0 |
| **Staging** | DigitalOcean | 2 vCPU, 4GB RAM | ~$24 |
| **Production (MVP)** | DigitalOcean | 8 vCPU, 16GB RAM | ~$96 |
| **Database** | Managed PostgreSQL | 2 vCPU, 4GB | ~$60 |
| **Redis** | Managed Redis | 2GB | ~$25 |
| **CDN** | Cloudflare | Pro Plan | ~$20 |
| **Total (MVP)** | | | ~$225/month |

---

## 8. Implementation Priority

### Phase 1: MVP (Month 1-2)

1. Authentication (Email + OAuth)
2. Server/Player creation
3. Village basics (resources, buildings)
4. Troop training
5. Army movement (raid/attack)
6. Basic map
7. Simple reports

### Phase 2: Core Features (Month 3-4)

1. Alliance system
2. Real-time updates (WebSocket)
3. Complete combat system
4. Market/trading
5. VIP subscription
6. Gold purchase

### Phase 3: Polish (Month 5-6)

1. Full i18n
2. Battle Pass
3. Skins system
4. Advanced alliance features
5. Performance optimization
6. Mobile optimization

---

*Document Version: 1.0*
*Last Updated: December 2025*
*Author: Technical Team*
