# Development Tasks: Tusk & Horn

## Role: Engineering Manager / Tech Lead
## Reference: Technical Plan (.docs/02-pland.md)

---

## Task Organization

Tasks are organized by **Phase** and **Module**, ordered by dependency.
Each task is small enough to be reviewed and tested independently.

**Legend:**
- [ ] Pending
- [x] Completed
- [~] Partially completed / Skipped

**Last Updated:** 2025-12-29 (synced with codebase)

---

# [x] Phase 1: Foundation & Infrastructure

## [x] 1.1 Project Setup

### [x] T001: Initialize Rust Backend Project
- **Description:** Create Rust project with Cargo and standard project structure following the backend layout in 02-pland.md.
- **Technical Context:**
  - Files: `backend/Cargo.toml`, `backend/src/main.rs`
  - Folders: `src/config`, `src/handlers`, `src/services`, `src/repositories`, `src/models`, `src/db`, `src/error`, `src/middleware`
- **Status:** Completed

---

### [x] T002: Initialize SvelteKit Frontend Project
- **Description:** Create SvelteKit project with TypeScript, Tailwind CSS, and svelte-i18n.
- **Technical Context:**
  - Files: `frontend/svelte.config.js`, `frontend/tailwind.config.js`
- **Status:** Completed

---

### [x] T003: Setup Docker Compose for Local Development
- **Description:** Create docker-compose.yml with PostgreSQL and Redis.
- **Technical Context:**
  - Files: `docker-compose.yml`
- **Status:** Completed

---

### [x] T004: Setup Configuration Management (Rust)
- **Description:** Implement configuration loading using dotenvy.
- **Technical Context:**
  - Files: `src/config/mod.rs`
- **Status:** Completed

---

### [x] T005: Setup PostgreSQL Connection Pool
- **Description:** Create database connection pool using SQLx.
- **Technical Context:**
  - Files: `src/db/postgres.rs`
- **Status:** Completed

---

### [x] T006: Setup Redis Connection
- **Description:** Create Redis client wrapper.
- **Technical Context:**
  - Files: `src/db/redis.rs`
- **Status:** Completed

---

### [x] T007: Setup Database Migrations
- **Description:** Create migration system using SQLx CLI.
- **Technical Context:**
  - Files: `migrations/*.sql` (34 migrations)
- **Status:** Completed

---

### [x] T008: Setup HTTP Server with Axum
- **Description:** Create HTTP server with Axum framework.
- **Technical Context:**
  - Files: `src/main.rs`
- **Status:** Completed

---

### [x] T009: Setup CORS Middleware
- **Description:** Implement CORS middleware.
- **Technical Context:**
  - Files: `src/main.rs`
- **Status:** Completed

---

### [x] T010: Setup Request Validation
- **Description:** Create request validation using serde.
- **Status:** Completed

---

### [x] T011: Setup Structured Logging
- **Description:** Implement structured logging with tracing.
- **Technical Context:**
  - Files: `src/main.rs`
- **Status:** Completed

---

### [x] T012: Setup Frontend API Client
- **Description:** Create API client service for frontend.
- **Technical Context:**
  - Files: `frontend/src/lib/api/client.ts`
- **Status:** Completed

---

### [x] T013: Setup Frontend WebSocket Client
- **Description:** Create WebSocket service for real-time updates.
- **Technical Context:**
  - Files: `frontend/src/lib/api/ws.ts`
- **Status:** Completed

---

### [x] T014: Create Makefile for Common Commands
- **Description:** Create Makefile with common development commands.
- **Technical Context:**
  - Files: `Makefile`
- **Status:** Completed

---

# [x] Phase 1: Database Schema

## [x] 1.2 Core Tables

### [x] T015: Create Users Table Migration
- **Description:** Create migration for users table with Firebase auth.
- **Technical Context:**
  - Files: `migrations/000012_update_users_auth.up.sql`, `migrations/000013_add_deleted_at_to_users.up.sql`
- **Status:** Completed

---

### [~] T016: Create Servers Table Migration
- **Description:** Create migration for game servers (worlds).
- **Status:** Skipped
- **Reason:** Using single server architecture

---

### [~] T017: Create Tribes Table Migration
- **Description:** Create migration for tribes reference table.
- **Status:** Skipped
- **Reason:** Tribe selection simplified to frontend-only

---

### [~] T018: Create Players Table Migration
- **Description:** Create migration for players (user in a server).
- **Status:** Skipped
- **Reason:** Users are directly players in single server

---

### [x] T019: Create Villages Table Migration
- **Description:** Create migration for villages with resources.
- **Technical Context:**
  - Files: `migrations/000014_create_villages.up.sql`
- **Status:** Completed

---

### [x] T020: Create Buildings Table Migration
- **Description:** Create migration for buildings and building queue.
- **Technical Context:**
  - Files: `migrations/000015_create_buildings.up.sql`
- **Status:** Completed

---

### [x] T021: Create Troops Tables Migration
- **Description:** Create migration for troop definitions, troops, and training queue.
- **Technical Context:**
  - Files: `migrations/000016_create_troops.up.sql`
- **Status:** Completed

---

### [x] T022: Create Armies Table Migration
- **Description:** Create migration for moving armies.
- **Technical Context:**
  - Files: `migrations/000017_create_armies.up.sql`, `migrations/000019_add_stationed_to_armies.up.sql`
- **Status:** Completed

---

### [x] T023: Create Alliances Tables Migration
- **Description:** Create migration for alliances, members, and diplomacy.
- **Technical Context:**
  - Files: `migrations/000022_create_alliances.up.sql`, `migrations/000023_create_alliance_diplomacy.up.sql`
- **Status:** Completed

---

### [x] T024: Create Reports Table Migration
- **Description:** Create migration for player reports (battle and scout reports).
- **Technical Context:**
  - Files: `migrations/000018_create_scout_reports.up.sql`
- **Status:** Completed

---

### [x] T025: Create Messages Table Migration
- **Description:** Create migration for private and alliance messages.
- **Technical Context:**
  - Files: `migrations/000024_create_messages.up.sql`
- **Status:** Completed

---

### [x] T026: Create Transactions Table Migration
- **Description:** Create migration for payment transactions.
- **Technical Context:**
  - Files: `migrations/000025_create_shop.up.sql`
- **Status:** Completed

---

### [~] T027: Create Map Tiles Table Migration
- **Description:** Create migration for terrain data.
- **Status:** Skipped
- **Reason:** Using villages table for map display instead of separate map_tiles

---

### [~] T028: Create Sessions Table Migration
- **Description:** Create migration for session storage.
- **Status:** Skipped
- **Reason:** Using Firebase Auth, no server-side sessions needed

---

### [~] T029: Seed Tribes Data
- **Description:** Create seed script to populate tribes table.
- **Status:** Skipped
- **Reason:** Tribe selection handled in frontend

---

### [x] T030: Seed Troop Definitions Data
- **Description:** Create seed script to populate troop_definitions.
- **Technical Context:**
  - Files: `migrations/000016_create_troops.up.sql`, `migrations/000020_add_chief_troops.up.sql`, `migrations/000021_seed_chief_troops.up.sql`
- **Status:** Completed

---

# [x] Phase 1: Authentication (Firebase)

## [x] 1.3 Auth System

### [x] T031: Setup Firebase Project & Admin SDK
- **Description:** Initialize Firebase project and setup JWT verification in Rust backend.
- **Technical Context:**
  - Files: `src/middleware/auth.rs`
- **Status:** Completed

---

### [x] T032: Create User Model and Repository
- **Description:** Implement User model linked to Firebase UID.
- **Technical Context:**
  - Files: `src/models/user.rs`, `src/repositories/user_repo.rs`
- **Status:** Completed

---

### [x] T033: Implement Auth Middleware (Firebase)
- **Description:** Create middleware to verify Firebase ID Token.
- **Technical Context:**
  - Files: `src/middleware/auth.rs`
- **Status:** Completed

---

### [x] T034: Implement Auth Handlers
- **Description:** Create handler to sync Firebase user to local DB.
- **Technical Context:**
  - Files: `src/handlers/auth.rs`
  - Endpoints: GET /api/auth/me, POST /api/auth/sync, PUT /api/auth/profile, DELETE /api/auth/account, DELETE /api/auth/logout
- **Status:** Completed

---

### [x] T035: Setup Firebase Client SDK (Frontend)
- **Description:** Initialize Firebase App in SvelteKit frontend.
- **Technical Context:**
  - Files: `frontend/src/lib/firebase/config.ts`
- **Status:** Completed

---

### [x] T036: Create Frontend Auth Store (Firebase)
- **Description:** Create Svelte store wrapping Firebase Auth state.
- **Technical Context:**
  - Files: `frontend/src/lib/stores/auth.ts`
- **Status:** Completed

---

### [x] T037: Create Login Page (Firebase)
- **Description:** Create login page with Firebase UI/Logic.
- **Technical Context:**
  - Files: `frontend/src/routes/+page.svelte` (landing page with login)
- **Status:** Completed

---

### [x] T038: Create Registration Page (Firebase)
- **Description:** Create registration page.
- **Status:** Completed (via Google OAuth)

---

### [x] T039: Create Protected Route Guard
- **Description:** Create route guard for authenticated-only pages.
- **Technical Context:**
  - Files: `frontend/src/routes/game/+layout.svelte`
- **Status:** Completed

---

# [~] Phase 1: Server & Player Management

## [~] 1.4 Server & Player

**Status:** Skipped - Using single server architecture

### [~] T044-T053: Server & Player System
- **Status:** Skipped
- **Reason:** Simplified to single server with users as players directly

---

# [x] Phase 1: Village System

## [x] 1.5 Village Management

### [x] T054: Create Village Model and Repository
- **Description:** Implement Village model and repository.
- **Technical Context:**
  - Files: `src/models/village.rs`, `src/repositories/village_repo.rs`
- **Status:** Completed

---

### [x] T055: Create Building Model and Repository
- **Description:** Implement Building model and repository.
- **Technical Context:**
  - Files: `src/models/building.rs`, `src/repositories/building_repo.rs`
- **Status:** Completed

---

### [x] T056: Create Building Definitions Data
- **Description:** Define building stats, costs, and requirements.
- **Technical Context:**
  - Files: `src/models/building.rs` (BuildingDefinition)
- **Status:** Completed

---

### [x] T057: Implement Village Service
- **Description:** Create service for village operations.
- **Technical Context:**
  - Files: `src/services/village_service.rs`
- **Status:** Completed

---

### [x] T058: Implement Building Service
- **Description:** Create service for building operations.
- **Technical Context:**
  - Files: `src/services/building_service.rs`
- **Status:** Completed

---

### [x] T059: Implement Village Handlers
- **Description:** Create HTTP handlers for village endpoints.
- **Technical Context:**
  - Files: `src/handlers/village.rs`
  - Endpoints: GET /api/villages, POST /api/villages, GET /api/villages/{id}, PUT /api/villages/{id}, GET /api/map
- **Status:** Completed

---

### [x] T060: Implement Building Handlers
- **Description:** Create HTTP handlers for building endpoints.
- **Technical Context:**
  - Files: `src/handlers/building.rs`
  - Endpoints: GET /api/villages/{id}/buildings, POST /api/villages/{id}/buildings/{slot}, POST /api/villages/{id}/buildings/{slot}/upgrade, DELETE /api/villages/{id}/buildings/{slot}
- **Status:** Completed

---

### [x] T061: Create Village Store (Frontend)
- **Description:** Create Svelte store for selected village.
- **Technical Context:**
  - Files: `frontend/src/lib/stores/village.ts`
- **Status:** Completed

---

### [x] T062: Create Resource Store (Frontend)
- **Description:** Create Svelte store for live resource updates.
- **Technical Context:**
  - Files: `frontend/src/lib/stores/village.ts` (combined)
- **Status:** Completed

---

### [x] T063: Create ResourceBar Component
- **Description:** Create reusable resource bar UI component.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/ResourceBar.svelte`
- **Status:** Completed

---

### [x] T064: Create Village Overview Page
- **Description:** Create main village view page.
- **Technical Context:**
  - Files: `frontend/src/routes/game/village/+page.svelte`
- **Status:** Completed

---

### [x] T065: Create BuildingSlot Component
- **Description:** Create component for building slot in village grid.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/BuildingSlot.svelte`
- **Status:** Completed

---

### [x] T066: Create Building Details Modal
- **Description:** Create modal for building information and actions.
- **Technical Context:**
  - Files: `frontend/src/lib/components/modals/BuildingDetailModal.svelte`
- **Status:** Completed

---

### [x] T067: Create Buildings List Page
- **Description:** Create page listing all buildings in village.
- **Technical Context:**
  - Files: `frontend/src/lib/components/modals/BuildMenuModal.svelte`
- **Status:** Completed

---

# [x] Phase 1: Troop System

## [x] 1.6 Troop Training

### [x] T068: Create Troop Model and Repository
- **Description:** Implement Troop and TroopQueue models.
- **Technical Context:**
  - Files: `src/models/troop.rs`, `src/repositories/troop_repo.rs`
- **Status:** Completed

---

### [x] T069: Create Troop Definitions Data
- **Description:** Define troop stats and costs.
- **Technical Context:**
  - Files: Database seeded via migrations
- **Status:** Completed

---

### [x] T070: Implement Troop Service
- **Description:** Create service for troop operations.
- **Technical Context:**
  - Files: `src/services/troop_service.rs`
- **Status:** Completed

---

### [x] T071: Implement Troop Handlers
- **Description:** Create HTTP handlers for troop endpoints.
- **Technical Context:**
  - Files: `src/handlers/troop.rs`
  - Endpoints: GET /api/troops/definitions, GET /api/villages/{id}/troops, POST /api/villages/{id}/troops/train, DELETE /api/villages/{id}/troops/queue/{id}
- **Status:** Completed

---

### [x] T072: Create TroopCard Component
- **Description:** Create component for displaying troop info.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/TroopCard.svelte`
- **Status:** Completed

---

### [x] T073: Create Troops Page
- **Description:** Create page for viewing and training troops.
- **Technical Context:**
  - Files: `frontend/src/lib/components/modals/TrainingModal.svelte`
- **Status:** Completed

---

### [x] T074: Create Timer Component
- **Description:** Create reusable countdown timer component.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/TrainingQueue.svelte`
- **Status:** Completed

---

# [x] Phase 1: Map System

## [x] 1.7 World Map

### [~] T075: Create MapTile Model and Repository
- **Description:** Implement MapTile model for terrain data.
- **Status:** Partial
- **Note:** Map shows villages only, no separate terrain system

---

### [ ] T076: Create Map Generation Script
- **Description:** Create script to generate map tiles for a server.
- **Status:** Not started

---

### [x] T077: Implement Map Service
- **Description:** Create service for map operations.
- **Technical Context:**
  - Files: `src/repositories/village_repo.rs` (find_villages_in_area)
- **Status:** Completed

---

### [x] T078: Implement Map Handlers
- **Description:** Create HTTP handlers for map endpoints.
- **Technical Context:**
  - Files: `src/handlers/village.rs`
  - Endpoints: GET /api/map
- **Status:** Completed

---

### [x] T079: Create MapTile Component
- **Description:** Create component for single map tile.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/MapTile.svelte`
- **Status:** Completed

---

### [x] T080: Create Map Page
- **Description:** Create world map page with viewport.
- **Technical Context:**
  - Files: `frontend/src/routes/game/map/+page.svelte`
- **Status:** Completed

---

### [ ] T081: Create Map Search Component
- **Description:** Create search box for finding locations.
- **Status:** Not started

---

### [x] T082: Create Tile Details Modal
- **Description:** Create modal for tile information and actions.
- **Technical Context:**
  - Files: `frontend/src/lib/components/modals/TileDetailModal.svelte`
- **Status:** Completed

---

# [x] Phase 1: Combat System

## [x] 1.8 Army Movement & Combat

### [x] T083: Create Army Model and Repository
- **Description:** Implement Army model for moving troops.
- **Technical Context:**
  - Files: `src/models/army.rs`, `src/repositories/army_repo.rs`
- **Status:** Completed

---

### [x] T084: Implement Combat Formulas
- **Description:** Create combat calculation formulas.
- **Technical Context:**
  - Files: `src/services/army_service.rs`
- **Status:** Completed
- **Note:** Includes hero passive bonuses in battle calculations

---

### [x] T085: Implement Army Service
- **Description:** Create service for army operations.
- **Technical Context:**
  - Files: `src/services/army_service.rs` (1257 lines)
- **Status:** Completed

---

### [x] T086: Implement Combat Service
- **Description:** Create service for battle execution.
- **Technical Context:**
  - Files: `src/services/army_service.rs` (combined)
- **Status:** Completed

---

### [x] T087: Implement Army Handlers
- **Description:** Create HTTP handlers for army endpoints.
- **Technical Context:**
  - Files: `src/handlers/army.rs`
  - Endpoints: POST /api/villages/{id}/armies, GET /api/villages/{id}/armies/outgoing, GET /api/villages/{id}/armies/incoming, GET /api/villages/{id}/stationed, POST /api/armies/{id}/recall, GET /api/reports/*, GET /api/scout-reports/*
- **Status:** Completed

---

### [x] T088: Create Army Overview Page
- **Description:** Create page showing all player armies.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/ArmyMovementPanel.svelte`
- **Status:** Completed

---

### [x] T089: Create Send Army Page
- **Description:** Create page for sending troops.
- **Technical Context:**
  - Files: `frontend/src/lib/components/modals/RallyPointModal.svelte`
- **Status:** Completed

---

### [x] T090: Create ArmyRow Component
- **Description:** Create component for army list item.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/ArmyList.svelte`
- **Status:** Completed

---

# [x] Phase 2: Game Engine

## [x] 2.1 Tick Engine

### [x] T091: Create Game Tick Engine
- **Description:** Implement main game tick engine that runs periodic updates.
- **Technical Context:**
  - Files: `src/services/background_jobs.rs`
- **Status:** Completed

---

### [x] T092: Implement Resource Tick
- **Description:** Create resource production tick.
- **Technical Context:**
  - Files: `src/services/background_jobs.rs`, `src/services/resource_service.rs`
- **Status:** Completed
- **Note:** Runs every 5 minutes, batch updates all villages

---

### [x] T093: Implement Building Tick
- **Description:** Create building completion tick.
- **Technical Context:**
  - Files: `src/services/background_jobs.rs`, `src/services/building_service.rs`
- **Status:** Completed
- **Note:** Runs every 10 seconds

---

### [x] T094: Implement Troop Training Tick
- **Description:** Create troop training completion tick.
- **Technical Context:**
  - Files: `src/services/background_jobs.rs`
- **Status:** Completed
- **Note:** Processes training queue completion

---

### [x] T095: Implement Army Arrival Tick
- **Description:** Create army arrival processing tick.
- **Technical Context:**
  - Files: `src/services/background_jobs.rs`, `src/services/army_service.rs`
- **Status:** Completed
- **Note:** Runs every 5 seconds

---

### [x] T096: Implement Starvation Tick
- **Description:** Create starvation processing tick.
- **Technical Context:**
  - Files: `src/services/background_jobs.rs`
- **Status:** Completed
- **Note:** TroopsStarved event broadcasts via WebSocket

---

# [x] Phase 2: Real-time System

## [x] 2.2 WebSocket & Notifications

### [x] T097: Create WebSocket Hub
- **Description:** Implement WebSocket connection hub in backend.
- **Technical Context:**
  - Files: `src/services/ws_service.rs`
- **Status:** Completed
- **Note:** WsManager with user connections and broadcasts

---

### [x] T098: Create WebSocket Client Handler
- **Description:** Implement WebSocket client connection handler.
- **Technical Context:**
  - Files: `src/handlers/ws.rs`
- **Status:** Completed

---

### [x] T099: Define WebSocket Events
- **Description:** Define all WebSocket event types and payloads.
- **Technical Context:**
  - Files: `src/services/ws_service.rs`
- **Status:** Completed
- **Events:** BuildingCompleted, TrainingCompleted, ArmyArrived, TroopsStarved, etc.

---

### [x] T100: Integrate WebSocket with Handlers
- **Description:** Add WebSocket endpoint to HTTP server.
- **Technical Context:**
  - Files: `src/main.rs`
  - Endpoint: GET /ws
- **Status:** Completed

---

### [x] T101: Integrate WebSocket with Game Engine
- **Description:** Trigger WebSocket events from game ticks.
- **Technical Context:**
  - Files: `src/services/background_jobs.rs`
- **Status:** Completed

---

### [x] T102: Create Notification Service
- **Description:** Create service for managing notifications.
- **Technical Context:**
  - Files: `src/services/ws_service.rs`
- **Status:** Completed

---

### [x] T103: Create Notification Store (Frontend)
- **Description:** Create Svelte store for notifications.
- **Technical Context:**
  - Files: `frontend/src/lib/api/ws.ts`
- **Status:** Completed

---

### [~] T104: Create Toast Component
- **Description:** Create toast notification component.
- **Status:** Partial
- **Note:** Using WebSocket events, toast UI needs polish

---

# [x] Phase 2: Alliance System

## [x] 2.3 Alliance Features

### [x] T105: Create Alliance Model and Repository
- **Description:** Implement Alliance, AllianceMember, and AllianceDiplomacy models.
- **Technical Context:**
  - Files: `src/models/alliance.rs`, `src/repositories/alliance_repo.rs`
- **Status:** Completed

---

### [x] T106: Implement Alliance Service
- **Description:** Create service for alliance operations.
- **Technical Context:**
  - Files: `src/services/alliance_service.rs`
- **Status:** Completed
- **Features:** Create, join, leave, invite, kick, diplomacy

---

### [x] T107: Implement Alliance Handlers
- **Description:** Create HTTP handlers for alliance endpoints.
- **Technical Context:**
  - Files: `src/handlers/alliance.rs`
  - Endpoints: POST /api/alliances, GET /api/alliances, GET /api/alliances/my, GET /api/alliances/{id}, PUT /api/alliances/{id}, DELETE /api/alliances/{id}, POST /api/alliances/{id}/invite, POST /api/alliances/leave, GET /api/alliances/invitations, POST /api/alliances/invitations/{id}/respond, GET /api/alliances/{id}/diplomacy, POST /api/alliances/{id}/diplomacy
- **Status:** Completed

---

### [ ] T108: Create Alliance Home Page (Frontend)
- **Description:** Create alliance overview page in frontend.
- **Status:** Not started

---

### [ ] T109: Create Alliance Members Page (Frontend)
- **Description:** Create alliance members management page.
- **Status:** Not started

---

### [ ] T110: Create Alliance Diplomacy Page (Frontend)
- **Description:** Create alliance diplomacy management page.
- **Status:** Not started

---

### [ ] T111: Create Alliance Chat Page (Frontend)
- **Description:** Create alliance chat/messages page.
- **Status:** Not started
- **Note:** Backend alliance messages API ready

---

# [x] Phase 2: Reports & Messages

## [x] 2.4 Communication

### [x] T112: Create Report Models
- **Description:** Implement Report models for battle and scout reports.
- **Technical Context:**
  - Files: `src/models/army.rs` (BattleReport, ScoutReport)
- **Status:** Completed

---

### [x] T113: Implement Report Repository
- **Description:** Create repository for report CRUD operations.
- **Technical Context:**
  - Files: `src/repositories/army_repo.rs`
- **Status:** Completed

---

### [x] T114: Implement Report Handlers
- **Description:** Create HTTP handlers for report endpoints.
- **Technical Context:**
  - Files: `src/handlers/army.rs`
  - Endpoints: GET /api/reports, GET /api/reports/{id}, POST /api/reports/{id}/read, GET /api/scout-reports, GET /api/scout-reports/{id}
- **Status:** Completed

---

### [x] T115: Create BattleReportCard Component
- **Description:** Create component for displaying battle reports.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/BattleReportCard.svelte`
- **Status:** Completed

---

### [x] T116: Create ScoutReportCard Component
- **Description:** Create component for displaying scout reports.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/ScoutReportCard.svelte`
- **Status:** Completed

---

### [x] T117: Create Message Model and Repository
- **Description:** Implement Message model for private and alliance messages.
- **Technical Context:**
  - Files: `src/models/message.rs`, `src/repositories/message_repo.rs`
- **Status:** Completed

---

### [x] T118: Implement Message Handlers
- **Description:** Create HTTP handlers for message endpoints.
- **Technical Context:**
  - Files: `src/handlers/message.rs`
  - Endpoints: POST /api/messages, GET /api/messages/inbox, GET /api/messages/sent, GET /api/messages/{id}, DELETE /api/messages/{id}, GET /api/conversations, GET /api/conversations/{id}/messages, POST /api/conversations/{id}/reply, POST /api/alliance-messages, GET /api/alliance-messages
- **Status:** Completed

---

### [ ] T119: Create Messages Page (Frontend)
- **Description:** Create messages inbox/compose page in frontend.
- **Status:** Not started
- **Note:** Backend API complete

---

# [x] Phase 2: Shop & Payment

## [x] 2.5 Monetization

### [x] T120: Create Transaction Model and Repository
- **Description:** Implement GoldTransaction and Subscription models.
- **Technical Context:**
  - Files: `src/models/shop.rs`, `src/repositories/shop_repo.rs`
- **Status:** Completed

---

### [x] T121: Implement Shop Service
- **Description:** Create service for shop operations.
- **Technical Context:**
  - Files: `src/services/shop_service.rs`
- **Status:** Completed
- **Features:** Gold packages, subscriptions, gold features

---

### [x] T122: Implement Payment Webhook Handler
- **Description:** Create Stripe webhook handler.
- **Technical Context:**
  - Files: `src/handlers/shop.rs`
  - Endpoint: POST /api/shop/webhook
- **Status:** Completed

---

### [x] T123: Implement Shop Handlers
- **Description:** Create HTTP handlers for shop endpoints.
- **Technical Context:**
  - Files: `src/handlers/shop.rs`
  - Endpoints: GET /api/shop/packages, GET /api/shop/subscriptions, GET /api/shop/balance, POST /api/shop/checkout, POST /api/shop/subscriptions/buy, GET /api/shop/transactions, POST /api/shop/features/*
- **Status:** Completed

---

### [ ] T124: Create Shop Page (Frontend)
- **Description:** Create shop page in frontend.
- **Status:** Not started
- **Note:** Backend API complete

---

### [x] T125: Implement VIP Benefits
- **Description:** Implement subscription benefits.
- **Technical Context:**
  - Files: `src/services/shop_service.rs`
- **Status:** Completed
- **Features:** Finish Now, NPC Merchant, Production Bonus, Book of Wisdom

---

# [x] Phase 2: Hero System (NEW)

## [x] 2.6 Hero Features

### [x] T141: Create Hero Model and Repository
- **Description:** Implement Hero model with attributes, inventory, and adventures.
- **Technical Context:**
  - Files: `src/models/hero.rs`, `src/repositories/hero_repo.rs`
- **Status:** Completed

---

### [x] T142: Create Hero Definitions Data
- **Description:** Define named heroes with passive bonuses.
- **Technical Context:**
  - Files: `src/models/hero.rs` (HeroDefinition)
- **Status:** Completed
- **Heroes:** 12 unique named heroes with passive bonuses

---

### [x] T143: Implement Hero Service
- **Description:** Create service for hero operations.
- **Technical Context:**
  - Files: `src/services/hero_service.rs`
- **Status:** Completed
- **Features:** CRUD, inventory, equip/unequip, adventures, revive, health regen

---

### [x] T144: Implement Hero Handlers
- **Description:** Create HTTP handlers for hero endpoints.
- **Technical Context:**
  - Files: `src/handlers/hero.rs`
  - Endpoints: GET /api/heroes, POST /api/heroes, GET /api/heroes/{id}, PUT /api/heroes/{id}/home, PUT /api/heroes/{id}/attributes, POST /api/heroes/slots/buy, GET /api/heroes/definitions, GET /api/heroes/tavern, GET /api/heroes/{id}/inventory, POST /api/heroes/{id}/equip, POST /api/heroes/{id}/unequip, POST /api/heroes/{id}/use-item, DELETE /api/heroes/{id}/items/{item_id}, GET /api/heroes/adventures/available, POST /api/heroes/{id}/adventures, GET /api/heroes/{id}/adventures/active, GET /api/heroes/{id}/revive-info, POST /api/heroes/{id}/revive
- **Status:** Completed

---

### [x] T145: Integrate Hero with Combat
- **Description:** Apply hero passive bonuses in battle calculations.
- **Technical Context:**
  - Files: `src/services/army_service.rs`
- **Status:** Completed

---

### [ ] T146: Create Hero Page (Frontend)
- **Description:** Create hero management page in frontend.
- **Status:** Not started
- **Note:** Backend API complete

---

# [x] Phase 2: Admin System (NEW)

## [x] 2.7 Admin Features

### [x] T147: Create Admin Model and Repository
- **Description:** Implement admin-related models and queries.
- **Technical Context:**
  - Files: `src/models/admin.rs`, `src/repositories/admin_repo.rs`
- **Status:** Completed

---

### [x] T148: Implement Admin Middleware
- **Description:** Create admin authorization middleware.
- **Technical Context:**
  - Files: `src/middleware/mod.rs` (admin_middleware)
- **Status:** Completed

---

### [x] T149: Implement Admin Service
- **Description:** Create service for admin operations.
- **Technical Context:**
  - Files: `src/services/admin_service.rs`
- **Status:** Completed
- **Features:** User management, ban/unban, server stats, resource adjustment

---

### [x] T150: Implement Admin Handlers
- **Description:** Create HTTP handlers for admin endpoints.
- **Technical Context:**
  - Files: `src/handlers/admin.rs`
  - Endpoints: GET /api/admin/users, GET /api/admin/users/search, GET /api/admin/users/{id}, POST /api/admin/users/{id}/ban, POST /api/admin/users/{id}/unban, PUT /api/admin/users/{id}/admin, GET /api/admin/stats, POST /api/admin/villages/{id}/resources
- **Status:** Completed

---

### [ ] T151: Create Admin Panel (Frontend)
- **Description:** Create admin dashboard in frontend.
- **Status:** Not started
- **Note:** Backend API complete

---

# [x] Phase 2: Ranking System (NEW)

## [x] 2.8 Leaderboard Features

### [x] T152: Create Ranking Model and Repository
- **Description:** Implement ranking queries and models.
- **Technical Context:**
  - Files: `src/models/ranking.rs`, `src/repositories/ranking_repo.rs`
- **Status:** Completed

---

### [x] T153: Implement Ranking Service
- **Description:** Create service for ranking calculations.
- **Technical Context:**
  - Files: `src/services/ranking_service.rs`
- **Status:** Completed
- **Rankings:** Population, Attackers, Defenders, Heroes, Alliances

---

### [x] T154: Implement Ranking Handlers
- **Description:** Create HTTP handlers for ranking endpoints.
- **Technical Context:**
  - Files: `src/handlers/ranking.rs`
  - Endpoints: GET /api/rankings/players/population, GET /api/rankings/players/attackers, GET /api/rankings/players/defenders, GET /api/rankings/heroes, GET /api/rankings/alliances
- **Status:** Completed
- **Note:** Public endpoints, no auth required

---

### [ ] T155: Create Leaderboard Page (Frontend)
- **Description:** Create leaderboard page in frontend.
- **Status:** Not started
- **Note:** Backend API complete

---

# [~] Phase 2: Trade System (NEW)

## [~] 2.9 Market Features

### [x] T156: Create Trade Model and Repository
- **Description:** Implement TradeOrder, TradeTransaction, and ResourceLock models.
- **Technical Context:**
  - Files: `src/models/trade.rs`, `src/repositories/trade_repo.rs`
  - Migrations: `000031-000034` (trade enums, orders, transactions, resource_locks)
- **Status:** Completed

---

### [x] T157: Implement Trade Service
- **Description:** Create service for trade operations.
- **Technical Context:**
  - Files: `src/services/trade_service.rs`
- **Status:** Completed
- **Features:** Create buy/sell orders, accept orders, cancel orders, market summary, escrow system

---

### [x] T158: Implement Trade Handlers
- **Description:** Create HTTP handlers for trade endpoints.
- **Technical Context:**
  - Files: `src/handlers/trade.rs`
  - Public Endpoints: GET /api/market/summary, GET /api/market/orders, GET /api/market/orders/{id}, GET /api/market/transactions
  - Authenticated Endpoints: POST /api/trade/orders, GET /api/trade/orders, POST /api/trade/orders/{id}/accept, POST /api/trade/orders/{id}/cancel, GET /api/trade/history
- **Status:** Completed

---

### [ ] T159: Implement Trade Background Job
- **Description:** Create background job for expired orders processing.
- **Status:** Not started

---

### [ ] T160: Create Market Page (Frontend)
- **Description:** Create market/trade page in frontend.
- **Status:** Not started
- **Note:** Backend API complete

---

# Phase 3: Polish & Optimization

## 3.1 Final Features

### [ ] T126: Implement Full i18n Support
- **Description:** Complete internationalization for all text.
- **Status:** Not started

---

### [ ] T127: Create Game Dashboard
- **Description:** Create main game dashboard with overview.
- **Status:** Not started

---

### [ ] T128: Create Settings Page
- **Description:** Create user settings page.
- **Status:** Not started

---

### [x] T129: Create Game Layout with Sidebar
- **Description:** Create game layout with navigation sidebar.
- **Technical Context:**
  - Files: `frontend/src/routes/game/+layout.svelte`
- **Status:** Completed

---

### [x] T130: Create Landing Page
- **Description:** Create public landing page.
- **Technical Context:**
  - Files: `frontend/src/routes/+page.svelte`
- **Status:** Completed

---

### [ ] T131: Implement Error Handling & Loading States
- **Description:** Add comprehensive error handling and loading states.
- **Status:** Not started

---

### [ ] T132: Performance Optimization - Backend
- **Description:** Optimize backend performance.
- **Status:** Not started

---

### [ ] T133: Performance Optimization - Frontend
- **Description:** Optimize frontend performance.
- **Status:** Not started

---

### [ ] T134: Security Audit & Hardening
- **Description:** Perform security audit and implement hardening.
- **Status:** Not started

---

### [ ] T135: Mobile Responsiveness
- **Description:** Ensure mobile-friendly UI.
- **Status:** Not started

---

### [ ] T136: Create E2E Test Suite
- **Description:** Create end-to-end test suite.
- **Status:** Not started

---

### [ ] T137: Setup CI/CD Pipeline
- **Description:** Setup continuous integration and deployment.
- **Status:** Not started

---

### [ ] T138: Create Production Dockerfile
- **Description:** Create production-ready Docker configuration.
- **Status:** Not started

---

### [ ] T139: Setup Monitoring & Alerting
- **Description:** Setup monitoring and alerting system.
- **Status:** Not started

---

### [ ] T140: Documentation & README
- **Description:** Create comprehensive documentation.
- **Status:** Not started

---

# Summary

| Phase | Module | Status | Completed/Total |
|-------|--------|--------|-----------------|
| 1.1 | Project Setup | Complete | 14/14 |
| 1.2 | Database Schema | Complete | 12/16 (4 skipped) |
| 1.3 | Authentication | Complete | 9/9 |
| 1.4 | Server & Player | Skipped | 0/10 |
| 1.5 | Village System | Complete | 14/14 |
| 1.6 | Troop System | Complete | 7/7 |
| 1.7 | Map System | Partial | 6/8 |
| 1.8 | Combat System | Complete | 8/8 |
| 2.1 | Game Engine | Complete | 6/6 |
| 2.2 | WebSocket | Complete | 7/8 |
| 2.3 | Alliance System | Partial | 3/7 (Backend done) |
| 2.4 | Reports & Messages | Partial | 7/8 (Backend done) |
| 2.5 | Shop & Payment | Partial | 5/6 (Backend done) |
| 2.6 | Hero System | Partial | 5/6 (Backend done) |
| 2.7 | Admin System | Partial | 4/5 (Backend done) |
| 2.8 | Ranking System | Partial | 3/4 (Backend done) |
| 2.9 | Trade System | Partial | 3/5 (Backend done) |
| 3 | Polish & Optimization | Partial | 2/15 |

**Progress Summary:**
- **Backend Completed:** ~95%
- **Frontend Completed:** ~60%
- **Overall Completed:** ~80%

**Core Gameplay Status:** Fully Functional
- Village, buildings, troops, army, combat: Complete
- Real-time WebSocket updates: Complete
- Alliance, messages, shop, hero, admin, ranking, trade: Backend complete

**Remaining Work:**
- Frontend pages for new features (Alliance, Messages, Shop, Hero, Admin, Ranking, Trade)
- Polish and optimization tasks
- Testing and deployment setup

---

*Document Version: 3.0*
*Last Updated: 2025-12-29*
*Synced with: Actual codebase*
