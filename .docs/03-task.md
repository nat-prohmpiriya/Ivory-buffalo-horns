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

**Last Updated:** 2025-12-25 (synced with codebase)

---

# [x] Phase 1: Foundation & Infrastructure

## [x] 1.1 Project Setup

### [x] T001: Initialize Rust Backend Project
- **Description:** Create Rust project with Cargo and standard project structure following the backend layout in 02-pland.md.
- **Technical Context:**
  - Files: `backend/Cargo.toml`, `backend/src/main.rs`
  - Folders: `src/config`, `src/handlers`, `src/services`, `src/repositories`, `src/models`, `src/db`, `src/error`, `src/middleware`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Cargo project initialized
  - [x] Folder structure created
  - [x] main.rs with Axum server

---

### [x] T002: Initialize SvelteKit Frontend Project
- **Description:** Create SvelteKit project with TypeScript, Tailwind CSS, and svelte-i18n.
- **Technical Context:**
  - Files: `frontend/svelte.config.js`, `frontend/tailwind.config.js`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] SvelteKit + TypeScript
  - [x] Tailwind CSS configured
  - [x] svelte-i18n with th/en locales
  - [x] shadcn-svelte components

---

### [x] T003: Setup Docker Compose for Local Development
- **Description:** Create docker-compose.yml with PostgreSQL and Redis.
- **Technical Context:**
  - Files: `docker-compose.yml`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] PostgreSQL service
  - [x] Redis service

---

### [x] T004: Setup Configuration Management (Rust)
- **Description:** Implement configuration loading using dotenvy.
- **Technical Context:**
  - Files: `src/config/mod.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Config struct with Database, Redis, Server sections
  - [x] Loads from environment variables

---

### [x] T005: Setup PostgreSQL Connection Pool
- **Description:** Create database connection pool using SQLx.
- **Technical Context:**
  - Files: `src/db/postgres.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] SQLx PgPool initialized
  - [x] Connection pool configured

---

### [x] T006: Setup Redis Connection
- **Description:** Create Redis client wrapper.
- **Technical Context:**
  - Files: `src/db/redis.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Redis ConnectionManager initialized

---

### [x] T007: Setup Database Migrations
- **Description:** Create migration system using SQLx CLI.
- **Technical Context:**
  - Files: `migrations/*.sql` (21 migrations)
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] SQLx migrations working
  - [x] 21 migrations created

---

### [x] T008: Setup HTTP Server with Axum
- **Description:** Create HTTP server with Axum framework.
- **Technical Context:**
  - Files: `src/main.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Axum server with routes
  - [x] Health check endpoint: GET /health
  - [x] TraceLayer for logging

---

### [x] T009: Setup CORS Middleware
- **Description:** Implement CORS middleware.
- **Technical Context:**
  - Files: `src/main.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] CorsLayer::permissive() configured

---

### [x] T010: Setup Request Validation
- **Description:** Create request validation using serde.
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Serde validation on DTOs

---

### [x] T011: Setup Structured Logging
- **Description:** Implement structured logging with tracing.
- **Technical Context:**
  - Files: `src/main.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] tracing-subscriber configured
  - [x] EnvFilter for log levels

---

### [x] T012: Setup Frontend API Client
- **Description:** Create API client service for frontend.
- **Technical Context:**
  - Files: `frontend/src/lib/api/client.ts`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Fetch-based API client
  - [x] Authorization header injection
  - [x] Error handling

---

### [x] T013: Setup Frontend WebSocket Client
- **Description:** Create WebSocket service for real-time updates.
- **Technical Context:**
  - Files: `frontend/src/lib/api/ws.ts`
- **Status:** ✅ Complete (Frontend only)
- **Actual Implementation:**
  - [x] WebSocket client with reconnection
  - [x] Event subscription system
  - [x] Svelte store integration
- **Note:** Backend WebSocket handler not yet implemented (T097-T101)

---

### [x] T014: Create Makefile for Common Commands
- **Description:** Create Makefile with common development commands.
- **Technical Context:**
  - Files: `Makefile`
- **Status:** ✅ Complete

---

# Phase 1: Database Schema

## 1.2 Core Tables

### [x] T015: Create Users Table Migration
- **Description:** Create migration for users table with Firebase auth.
- **Technical Context:**
  - Files: `migrations/000012_update_users_auth.up.sql`, `migrations/000013_add_deleted_at_to_users.up.sql`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Users table with firebase_uid, email
  - [x] deleted_at for soft delete

---

### [~] T016: Create Servers Table Migration
- **Description:** Create migration for game servers (worlds).
- **Status:** ⏭️ Skipped
- **Reason:** Using single server architecture

---

### [~] T017: Create Tribes Table Migration
- **Description:** Create migration for tribes reference table.
- **Status:** ⏭️ Skipped
- **Reason:** Tribe selection simplified to frontend-only

---

### [~] T018: Create Players Table Migration
- **Description:** Create migration for players (user in a server).
- **Status:** ⏭️ Skipped
- **Reason:** Users are directly players in single server

---

### [x] T019: Create Villages Table Migration
- **Description:** Create migration for villages with resources.
- **Technical Context:**
  - Files: `migrations/000014_create_villages.up.sql`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Villages table with resources (wood, clay, iron, crop)
  - [x] Coordinates (x, y)
  - [x] Foreign key to users
  - [x] Loyalty system

---

### [x] T020: Create Buildings Table Migration
- **Description:** Create migration for buildings and building queue.
- **Technical Context:**
  - Files: `migrations/000015_create_buildings.up.sql`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Buildings table with slot, level, type
  - [x] Upgrade timestamps (upgrade_started_at, upgrade_ends_at)
  - [x] Foreign key to villages

---

### [x] T021: Create Troops Tables Migration
- **Description:** Create migration for troop definitions, troops, and training queue.
- **Technical Context:**
  - Files: `migrations/000016_create_troops.up.sql`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] troop_definitions table with all stats
  - [x] troops table for village garrison
  - [x] troop_queue table for training
  - [x] Seeded with all troop types

---

### [x] T022: Create Armies Table Migration
- **Description:** Create migration for moving armies.
- **Technical Context:**
  - Files: `migrations/000017_create_armies.up.sql`, `migrations/000019_add_stationed_to_armies.up.sql`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] mission_type enum (raid, attack, conquer, support, scout, settle, return)
  - [x] Troops as JSONB
  - [x] Resources carried as JSONB
  - [x] is_stationed for support troops
  - [x] is_returning flag

---

### [ ] T023: Create Alliances Tables Migration
- **Description:** Create migration for alliances, members, and diplomacy.
- **Status:** ❌ Not started

---

### [x] T024: Create Reports Table Migration
- **Description:** Create migration for player reports (scout reports).
- **Technical Context:**
  - Files: `migrations/000018_create_scout_reports.up.sql`
- **Status:** ✅ Complete (Scout reports only)
- **Actual Implementation:**
  - [x] scout_reports table
  - [x] Defender troops/buildings as JSONB
  - [x] is_read flag
- **Note:** Battle reports are inline in army processing, not persisted yet

---

### [ ] T025: Create Messages Table Migration
- **Description:** Create migration for private and alliance messages.
- **Status:** ❌ Not started

---

### [ ] T026: Create Transactions Table Migration
- **Description:** Create migration for payment transactions.
- **Status:** ❌ Not started

---

### [~] T027: Create Map Tiles Table Migration
- **Description:** Create migration for terrain data.
- **Status:** ⏭️ Skipped
- **Reason:** Using villages table for map display instead of separate map_tiles

---

### [~] T028: Create Sessions Table Migration
- **Description:** Create migration for session storage.
- **Status:** ⏭️ Skipped
- **Reason:** Using Firebase Auth, no server-side sessions needed

---

### [~] T029: Seed Tribes Data
- **Description:** Create seed script to populate tribes table.
- **Status:** ⏭️ Skipped
- **Reason:** Tribe selection handled in frontend

---

### [x] T030: Seed Troop Definitions Data
- **Description:** Create seed script to populate troop_definitions.
- **Technical Context:**
  - Files: `migrations/000016_create_troops.up.sql`, `migrations/000020_add_chief_troops.up.sql`, `migrations/000021_seed_chief_troops.up.sql`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] All troop types seeded (including Chief units)
  - [x] Stats: attack, defense_infantry, defense_cavalry, speed, carry, upkeep
  - [x] Training costs and times

---

# [x] Phase 1: Authentication (Firebase)

## 1.3 Auth System

### [x] T031: Setup Firebase Project & Admin SDK
- **Description:** Initialize Firebase project and setup JWT verification in Rust backend.
- **Technical Context:**
  - Files: `src/middleware/auth.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Firebase token verification via JWT
  - [x] Public keys fetched from Google

---

### [x] T032: Create User Model and Repository
- **Description:** Implement User model linked to Firebase UID.
- **Technical Context:**
  - Files: `src/models/user.rs`, `src/repositories/user_repo.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] User struct with id, firebase_uid, email, display_name
  - [x] find_by_firebase_uid method
  - [x] create_from_firebase method
  - [x] update and soft_delete methods

---

### [x] T033: Implement Auth Middleware (Firebase)
- **Description:** Create middleware to verify Firebase ID Token.
- **Technical Context:**
  - Files: `src/middleware/auth.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Extract Bearer token from Authorization header
  - [x] Verify JWT signature
  - [x] Extract Firebase UID
  - [x] Set user in request extensions

---

### [x] T034: Implement Auth Handlers
- **Description:** Create handler to sync Firebase user to local DB.
- **Technical Context:**
  - Files: `src/handlers/auth.rs`
  - Endpoints: GET /api/auth/me, POST /api/auth/sync, PUT /api/auth/profile, DELETE /api/auth/account, DELETE /api/auth/logout
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] GET /api/auth/me - Get current user
  - [x] POST /api/auth/sync - Sync Firebase user to DB
  - [x] PUT /api/auth/profile - Update profile
  - [x] DELETE /api/auth/account - Soft delete
  - [x] DELETE /api/auth/logout - Logout (client-side)

---

### [x] T035: Setup Firebase Client SDK (Frontend)
- **Description:** Initialize Firebase App in SvelteKit frontend.
- **Technical Context:**
  - Files: `frontend/src/lib/firebase/config.ts`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Firebase config from env
  - [x] Firebase App initialized
  - [x] Auth instance exported

---

### [x] T036: Create Frontend Auth Store (Firebase)
- **Description:** Create Svelte store wrapping Firebase Auth state.
- **Technical Context:**
  - Files: `frontend/src/lib/stores/auth.ts`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Listen to onAuthStateChanged
  - [x] Store user object and ID Token
  - [x] loginWithGoogle, logout handlers

---

### [x] T037: Create Login Page (Firebase)
- **Description:** Create login page with Firebase UI/Logic.
- **Technical Context:**
  - Files: `frontend/src/routes/+page.svelte` (landing page with login)
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Login with Google button
  - [x] Redirect to /game on success
- **Note:** Email/password login not implemented (Google OAuth only)

---

### [x] T038: Create Registration Page (Firebase)
- **Description:** Create registration page.
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Registration via Google OAuth (same as login)
- **Note:** Email/password registration not implemented

---

### [x] T039: Create Protected Route Guard
- **Description:** Create route guard for authenticated-only pages.
- **Technical Context:**
  - Files: `frontend/src/routes/game/+layout.svelte`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Check auth store
  - [x] Redirect to / if not authenticated

---

# [~] Phase 1: Server & Player Management

## 1.4 Server & Player

**Status:** ⏭️ Entire section skipped - Using single server architecture

### [~] T044-T053: Server & Player System
- **Status:** ⏭️ Skipped
- **Reason:** Simplified to single server with users as players directly

---

# [x] Phase 1: Village System

## 1.5 Village Management

### [x] T054: Create Village Model and Repository
- **Description:** Implement Village model and repository.
- **Technical Context:**
  - Files: `src/models/village.rs`, `src/repositories/village_repo.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Village struct with resources as f64
  - [x] create, find_by_id, find_by_user methods
  - [x] update_resources method
  - [x] find_villages_in_area for map

---

### [x] T055: Create Building Model and Repository
- **Description:** Implement Building model and repository.
- **Technical Context:**
  - Files: `src/models/building.rs`, `src/repositories/building_repo.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Building struct with level, type, upgrade status
  - [x] find_by_village, find_by_id methods
  - [x] start_upgrade, complete_upgrade methods
  - [x] find_completed_upgrades for background job

---

### [x] T056: Create Building Definitions Data
- **Description:** Define building stats, costs, and requirements.
- **Technical Context:**
  - Files: `src/models/building.rs` (BuildingDefinition)
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] BuildingDefinition with base stats
  - [x] Cost calculation formulas
  - [x] Build time calculation
  - [x] All 23 building types defined

---

### [x] T057: Implement Village Service
- **Description:** Create service for village operations.
- **Technical Context:**
  - Files: `src/services/village_service.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] create_starting_village with random position
  - [x] get_village_details with buildings
  - [x] update_resources with production rates

---

### [x] T058: Implement Building Service
- **Description:** Create service for building operations.
- **Technical Context:**
  - Files: `src/services/building_service.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] start_upgrade with resource validation
  - [x] complete_upgrade with level update
  - [x] get_build_queue

---

### [x] T059: Implement Village Handlers
- **Description:** Create HTTP handlers for village endpoints.
- **Technical Context:**
  - Files: `src/handlers/village.rs`
  - Endpoints: GET /api/villages, POST /api/villages, GET /api/villages/{id}, PUT /api/villages/{id}, GET /api/map
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] list_villages - List user's villages
  - [x] create_village - Create new village
  - [x] get_village - Get village details
  - [x] update_village - Rename village
  - [x] get_map - Get villages in viewport

---

### [x] T060: Implement Building Handlers
- **Description:** Create HTTP handlers for building endpoints.
- **Technical Context:**
  - Files: `src/handlers/building.rs`
  - Endpoints: GET /api/villages/{id}/buildings, POST /api/villages/{id}/buildings/{slot}, POST /api/villages/{id}/buildings/{slot}/upgrade, DELETE /api/villages/{id}/buildings/{slot}
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] list_buildings
  - [x] build - Build new building
  - [x] upgrade - Upgrade existing building
  - [x] demolish - Remove building
  - [x] get_build_queue

---

### [x] T061: Create Village Store (Frontend)
- **Description:** Create Svelte store for selected village.
- **Technical Context:**
  - Files: `frontend/src/lib/stores/village.ts`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Current village with resources
  - [x] Buildings list
  - [x] Selected village ID
  - [x] fetch and refresh methods

---

### [x] T062: Create Resource Store (Frontend)
- **Description:** Create Svelte store for live resource updates.
- **Technical Context:**
  - Files: `frontend/src/lib/stores/village.ts` (combined)
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Resources in village store
  - [x] Production rates calculated
- **Note:** WebSocket updates not yet implemented

---

### [x] T063: Create ResourceBar Component
- **Description:** Create reusable resource bar UI component.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/ResourceBar.svelte`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Display 4 resource types with icons
  - [x] Show current/max capacity
  - [x] Production rate per hour

---

### [x] T064: Create Village Overview Page
- **Description:** Create main village view page.
- **Technical Context:**
  - Files: `frontend/src/routes/game/village/+page.svelte`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Village name and coordinates
  - [x] Resource bar at top
  - [x] Building slots grid
  - [x] Army movement panel
  - [x] Stationed troops panel

---

### [x] T065: Create BuildingSlot Component
- **Description:** Create component for building slot in village grid.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/BuildingSlot.svelte`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Display building icon and level
  - [x] Show upgrade progress
  - [x] Click to open details

---

### [x] T066: Create Building Details Modal
- **Description:** Create modal for building information and actions.
- **Technical Context:**
  - Files: `frontend/src/lib/components/modals/BuildingDetailModal.svelte`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Building name, level, description
  - [x] Upgrade button with cost
  - [x] Production stats for resource buildings
  - [x] Special actions (Barracks -> train, Rally Point -> send army)

---

### [x] T067: Create Buildings List Page
- **Description:** Create page listing all buildings in village.
- **Technical Context:**
  - Files: `frontend/src/lib/components/modals/BuildMenuModal.svelte`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Build menu modal with all building types
  - [x] Filter by category
  - [x] Requirements check

---

# [x] Phase 1: Troop System

## 1.6 Troop Training

### [x] T068: Create Troop Model and Repository
- **Description:** Implement Troop and TroopQueue models.
- **Technical Context:**
  - Files: `src/models/troop.rs`, `src/repositories/troop_repo.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Troop struct with count per type
  - [x] TroopQueue struct for training
  - [x] TroopDefinition from database
  - [x] CRUD operations

---

### [x] T069: Create Troop Definitions Data
- **Description:** Define troop stats and costs.
- **Technical Context:**
  - Files: Database seeded via migrations
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] All troop types in database
  - [x] Stats: attack, defense, speed, carry, upkeep
  - [x] Training time and costs
  - [x] Chief units for conquest

---

### [x] T070: Implement Troop Service
- **Description:** Create service for troop operations.
- **Technical Context:**
  - Files: `src/services/troop_service.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] get_village_troops
  - [x] start_training with resource validation
  - [x] cancel_training with refund
  - [x] get_training_queue

---

### [x] T071: Implement Troop Handlers
- **Description:** Create HTTP handlers for troop endpoints.
- **Technical Context:**
  - Files: `src/handlers/troop.rs`
  - Endpoints: GET /api/troops/definitions, GET /api/villages/{id}/troops, POST /api/villages/{id}/troops/train, DELETE /api/villages/{id}/troops/queue/{id}
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] get_definitions (public)
  - [x] list_troops
  - [x] train_troops
  - [x] cancel_training
  - [x] get_training_queue

---

### [x] T072: Create TroopCard Component
- **Description:** Create component for displaying troop info.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/TroopCard.svelte`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Troop icon and name
  - [x] Stats display (attack, defense, speed, carry)
  - [x] Training cost
  - [x] Train button with quantity input

---

### [x] T073: Create Troops Page
- **Description:** Create page for viewing and training troops.
- **Technical Context:**
  - Files: `frontend/src/lib/components/modals/TrainingModal.svelte`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Training modal from Barracks
  - [x] List available troops
  - [x] Current garrison count
  - [x] Training queue with timer

---

### [x] T074: Create Timer Component
- **Description:** Create reusable countdown timer component.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/TrainingQueue.svelte`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Countdown timer in training queue
  - [x] Format HH:MM:SS

---

# [x] Phase 1: Map System

## 1.7 World Map

### [~] T075: Create MapTile Model and Repository
- **Description:** Implement MapTile model for terrain data.
- **Status:** ⚠️ Partial
- **Actual Implementation:**
  - [x] Using villages table for map display
  - [ ] No separate terrain tiles
- **Note:** Map shows villages only, no terrain system

---

### [ ] T076: Create Map Generation Script
- **Description:** Create script to generate map tiles for a server.
- **Status:** ❌ Not started

---

### [~] T077: Implement Map Service
- **Description:** Create service for map operations.
- **Technical Context:**
  - Files: `src/repositories/village_repo.rs` (find_villages_in_area)
- **Status:** ⚠️ Partial
- **Actual Implementation:**
  - [x] Get villages in viewport
  - [ ] No terrain data
  - [ ] No search functionality

---

### [x] T078: Implement Map Handlers
- **Description:** Create HTTP handlers for map endpoints.
- **Technical Context:**
  - Files: `src/handlers/village.rs`
  - Endpoints: GET /api/map
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] get_map with viewport params

---

### [x] T079: Create MapTile Component
- **Description:** Create component for single map tile.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/MapTile.svelte`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Display village if present
  - [x] Owner indicator (self, other)
  - [x] Click to open tile actions

---

### [x] T080: Create Map Page
- **Description:** Create world map page with viewport.
- **Technical Context:**
  - Files: `frontend/src/routes/game/map/+page.svelte`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Grid display of tiles
  - [x] Pan with arrow keys
  - [x] Coordinate display

---

### [ ] T081: Create Map Search Component
- **Description:** Create search box for finding locations.
- **Status:** ❌ Not started

---

### [x] T082: Create Tile Details Modal
- **Description:** Create modal for tile information and actions.
- **Technical Context:**
  - Files: `frontend/src/lib/components/modals/TileDetailModal.svelte`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Village info if present
  - [x] Send army button
  - [x] Distance from current village

---

# [x] Phase 1: Combat System

## 1.8 Army Movement & Combat

### [x] T083: Create Army Model and Repository
- **Description:** Implement Army model for moving troops.
- **Technical Context:**
  - Files: `src/models/army.rs`, `src/repositories/army_repo.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Army struct with JSONB troops/resources
  - [x] Mission types: raid, attack, conquer, support, scout, settle, return
  - [x] find_by_player, find_arriving, find_incoming methods
  - [x] Stationed troops support

---

### [x] T084: Implement Combat Formulas
- **Description:** Create combat calculation formulas.
- **Technical Context:**
  - Files: `src/services/army_service.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Attack power calculation
  - [x] Defense power calculation
  - [x] Troop loss calculation
  - [x] Resource capture calculation
  - [x] Loyalty reduction for conquest

---

### [x] T085: Implement Army Service
- **Description:** Create service for army operations.
- **Technical Context:**
  - Files: `src/services/army_service.rs` (1257 lines)
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] send_army with all mission types
  - [x] process_arrived_armies (background job)
  - [x] recall_support
  - [x] Combat simulation
  - [x] Scout reports
  - [x] Conquest with Chief

---

### [x] T086: Implement Combat Service
- **Description:** Create service for battle execution.
- **Technical Context:**
  - Files: `src/services/army_service.rs` (combined)
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Raid: steal resources
  - [x] Attack: destroy troops, lower loyalty
  - [x] Support: station troops
  - [x] Scout: get enemy info
  - [x] Conquer: capture village with Chief

---

### [x] T087: Implement Army Handlers
- **Description:** Create HTTP handlers for army endpoints.
- **Technical Context:**
  - Files: `src/handlers/army.rs`
  - Endpoints: POST /api/villages/{id}/armies, GET /api/villages/{id}/armies/outgoing, GET /api/villages/{id}/armies/incoming, GET /api/villages/{id}/stationed, POST /api/armies/{id}/recall, GET /api/reports/*, GET /api/scout-reports/*
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] send_army
  - [x] list_outgoing, list_incoming
  - [x] list_stationed
  - [x] recall_support
  - [x] list_reports, get_report, mark_read
  - [x] list_scout_reports, get_scout_report

---

### [x] T088: Create Army Overview Page
- **Description:** Create page showing all player armies.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/ArmyMovementPanel.svelte`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Outgoing armies panel
  - [x] Incoming armies panel
  - [x] Arrival countdown

---

### [x] T089: Create Send Army Page
- **Description:** Create page for sending troops.
- **Technical Context:**
  - Files: `frontend/src/lib/components/modals/RallyPointModal.svelte`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Target coordinates input
  - [x] Mission type selection (raid, attack, support, scout, conquer)
  - [x] Troop selection
  - [x] Travel time preview

---

### [x] T090: Create ArmyRow Component
- **Description:** Create component for army list item.
- **Technical Context:**
  - Files: `frontend/src/lib/components/game/ArmyList.svelte`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Source and destination
  - [x] Mission type icon
  - [x] Troop summary
  - [x] Arrival countdown

---

# Phase 2: Game Engine

## 2.1 Tick Engine

### [x] T091: Create Game Tick Engine
- **Description:** Implement main game tick engine that runs periodic updates.
- **Technical Context:**
  - Files: `src/services/background_jobs.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Tokio spawn for background tasks
  - [x] Three separate tick jobs
  - [x] Error recovery and logging

---

### [x] T092: Implement Resource Tick
- **Description:** Create resource production tick.
- **Technical Context:**
  - Files: `src/services/background_jobs.rs`, `src/services/resource_service.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Runs every 5 minutes
  - [x] Batch update all villages
  - [x] Calculate production based on building levels
- **Note:** No WebSocket notification yet

---

### [x] T093: Implement Building Tick
- **Description:** Create building completion tick.
- **Technical Context:**
  - Files: `src/services/background_jobs.rs`, `src/services/building_service.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Runs every 10 seconds
  - [x] Check buildings with ends_at <= now
  - [x] Complete upgrade, update level
- **Note:** No WebSocket notification yet

---

### [ ] T094: Implement Troop Training Tick
- **Description:** Create troop training completion tick.
- **Status:** ❌ Not started
- **Note:** Training queue completion not automated yet

---

### [x] T095: Implement Army Arrival Tick
- **Description:** Create army arrival processing tick.
- **Technical Context:**
  - Files: `src/services/background_jobs.rs`, `src/services/army_service.rs`
- **Status:** ✅ Complete
- **Actual Implementation:**
  - [x] Runs every 5 seconds
  - [x] Process arrived armies
  - [x] Execute combat/support/scout
  - [x] Create returning army

---

### [ ] T096: Implement Starvation Tick
- **Description:** Create starvation processing tick.
- **Status:** ❌ Not started

---

# Phase 2: Real-time System

## 2.2 WebSocket & Notifications

### [ ] T097: Create WebSocket Hub
- **Description:** Implement WebSocket connection hub in backend.
- **Status:** ❌ Not started
- **Note:** Frontend client ready (T013), backend handler needed

---

### [ ] T098: Create WebSocket Client Handler
- **Description:** Implement WebSocket client connection handler.
- **Status:** ❌ Not started

---

### [ ] T099: Define WebSocket Events
- **Description:** Define all WebSocket event types and payloads.
- **Status:** ❌ Not started

---

### [ ] T100: Integrate WebSocket with Handlers
- **Description:** Add WebSocket endpoint to HTTP server.
- **Status:** ❌ Not started

---

### [ ] T101: Integrate WebSocket with Game Engine
- **Description:** Trigger WebSocket events from game ticks.
- **Status:** ❌ Not started

---

### [ ] T102: Create Notification Service
- **Description:** Create service for managing notifications.
- **Status:** ❌ Not started

---

### [ ] T103: Create Notification Store (Frontend)
- **Description:** Create Svelte store for notifications.
- **Status:** ❌ Not started

---

### [ ] T104: Create Toast Component
- **Description:** Create toast notification component.
- **Status:** ❌ Not started

---

# Phase 2: Alliance System

## 2.3 Alliance Features

### [ ] T105-T111: Alliance System
- **Status:** ❌ Not started
- **Tasks:**
  - [ ] T105: Create Alliance Model and Repository
  - [ ] T106: Implement Alliance Service
  - [ ] T107: Implement Alliance Handlers
  - [ ] T108: Create Alliance Home Page
  - [ ] T109: Create Alliance Members Page
  - [ ] T110: Create Alliance Diplomacy Page
  - [ ] T111: Create Alliance Chat Page

---

# Phase 2: Reports & Messages

## 2.4 Communication

### [~] T112-T116: Reports System
- **Status:** ⚠️ Partial
- **Actual Implementation:**
  - [x] Scout reports (T112-T116 partial)
  - [ ] Battle reports persistence
  - [ ] Trade reports
- **Tasks Completed:**
  - [x] Scout report model and repository
  - [x] Scout report handlers
  - [x] ScoutReportCard component
  - [x] ReportsModal for viewing reports

---

### [ ] T117-T119: Messages System
- **Status:** ❌ Not started
- **Tasks:**
  - [ ] T117: Create Message Model and Repository
  - [ ] T118: Implement Message Handlers
  - [ ] T119: Create Messages Page

---

# Phase 2: Shop & Payment

## 2.5 Monetization

### [ ] T120-T125: Shop & Payment System
- **Status:** ❌ Not started
- **Tasks:**
  - [ ] T120: Create Transaction Model and Repository
  - [ ] T121: Implement Shop Service
  - [ ] T122: Implement Payment Webhook Handler
  - [ ] T123: Implement Shop Handlers
  - [ ] T124: Create Shop Page
  - [ ] T125: Implement VIP Benefits

---

# Phase 3: Polish & Optimization

## 3.1 Final Features

### [ ] T126-T140: Polish & Optimization
- **Status:** ❌ Not started
- **Tasks:**
  - [ ] T126: Implement Full i18n Support
  - [ ] T127: Create Game Dashboard
  - [ ] T128: Create Settings Page
  - [ ] T129: Create Game Layout with Sidebar
  - [ ] T130: Create Landing Page
  - [ ] T131: Implement Error Handling & Loading States
  - [ ] T132: Performance Optimization - Backend
  - [ ] T133: Performance Optimization - Frontend
  - [ ] T134: Security Audit & Hardening
  - [ ] T135: Mobile Responsiveness
  - [ ] T136: Create E2E Test Suite
  - [ ] T137: Setup CI/CD Pipeline
  - [ ] T138: Create Production Dockerfile
  - [ ] T139: Setup Monitoring & Alerting
  - [ ] T140: Documentation & README

---

# Summary

| Phase | Module | Status | Completed/Total |
|-------|--------|--------|-----------------|
| 1.1 | Project Setup | ✅ Complete | 14/14 |
| 1.2 | Database Schema | ⚠️ Partial | 7/16 (5 skipped) |
| 1.3 | Authentication | ✅ Complete | 9/9 |
| 1.4 | Server & Player | ⏭️ Skipped | 0/10 |
| 1.5 | Village System | ✅ Complete | 14/14 |
| 1.6 | Troop System | ✅ Complete | 7/7 |
| 1.7 | Map System | ⚠️ Partial | 5/8 |
| 1.8 | Combat System | ✅ Complete | 8/8 |
| 2.1 | Game Engine | ⚠️ Partial | 4/6 |
| 2.2 | WebSocket | ❌ Not started | 0/8 |
| 2.3 | Alliance System | ❌ Not started | 0/7 |
| 2.4 | Reports & Messages | ⚠️ Partial | 3/8 |
| 2.5 | Shop & Payment | ❌ Not started | 0/6 |
| 3 | Polish & Optimization | ❌ Not started | 0/15 |

**Progress Summary:**
- **Completed:** ~71 tasks
- **Partial:** ~8 tasks
- **Skipped:** ~15 tasks
- **Pending:** ~46 tasks
- **Total:** 140 tasks

**Core Gameplay Status:** ✅ Functional
- Village management, building, troops, army combat all working
- Missing: Real-time updates, social features, monetization

---

*Document Version: 2.0*
*Last Updated: 2025-12-25*
*Synced with: Actual codebase*
