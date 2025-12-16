# Tusk & Horn

A real-time strategy browser game inspired by Southeast Asian ancient kingdoms. Build your empire, train legendary armies, and conquer the land.

## Overview

Tusk & Horn is a Travian-style strategy game featuring three unique tribes based on Southeast Asian civilizations:

- **Phasuttha** (The Mainland Empire) - Masters of war elephants and diplomacy
- **Nava** (The Maritime Raiders) - Swift raiders from the archipelago
- **Kiri** (The Highland Warriors) - Fierce defenders of the mountains

## Tech Stack

### Frontend
- **Framework:** SvelteKit 2 + Svelte 5
- **UI Components:** shadcn-svelte
- **Styling:** Tailwind CSS 4
- **Authentication:** Firebase Auth (Google OAuth)
- **Language:** TypeScript

### Backend
- **Framework:** Rust + Axum
- **Database:** PostgreSQL 15
- **Cache:** Redis 7
- **Real-time:** WebSocket

## Project Structure

```
travillian-games/
├── frontend/                 # SvelteKit frontend
│   ├── src/
│   │   ├── lib/
│   │   │   ├── components/   # UI components
│   │   │   │   ├── ui/       # shadcn-svelte components
│   │   │   │   ├── game/     # Game-specific components
│   │   │   │   └── modals/   # Modal dialogs
│   │   │   ├── stores/       # Svelte stores
│   │   │   ├── services/     # API & WebSocket clients
│   │   │   ├── firebase/     # Firebase config
│   │   │   └── i18n/         # Internationalization
│   │   └── routes/           # SvelteKit routes
│   │       ├── game/         # Game pages (village, map)
│   │       └── onboarding/   # Tribe selection
│   └── package.json
├── backend/                  # Rust backend
│   ├── src/
│   │   ├── handlers/         # HTTP handlers
│   │   ├── services/         # Business logic
│   │   ├── repositories/     # Database operations
│   │   ├── models/           # Data models
│   │   ├── game/             # Game engine
│   │   └── realtime/         # WebSocket
│   ├── migrations/           # SQL migrations
│   └── Cargo.toml
├── docker-compose.yml        # Local development services
└── .docs/                    # Project documentation
```

## Getting Started

### Prerequisites

- Node.js 20+
- Rust 1.75+
- Docker & Docker Compose
- Firebase project (for authentication)

### Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd travillian-games
   ```

2. **Start infrastructure services**
   ```bash
   docker-compose up -d
   ```

3. **Setup Frontend**
   ```bash
   cd frontend
   npm install
   cp .env.example .env
   # Edit .env with your Firebase credentials
   npm run dev
   ```

4. **Setup Backend**
   ```bash
   cd backend
   cp .env.example .env
   # Edit .env with database credentials
   cargo run
   ```

### Environment Variables

#### Frontend (.env)
```env
VITE_FIREBASE_API_KEY=your-api-key
VITE_FIREBASE_AUTH_DOMAIN=your-project.firebaseapp.com
VITE_FIREBASE_PROJECT_ID=your-project-id
VITE_FIREBASE_STORAGE_BUCKET=your-project.appspot.com
VITE_FIREBASE_MESSAGING_SENDER_ID=123456789
VITE_FIREBASE_APP_ID=1:123456789:web:abc123
```

#### Backend (.env)
```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/tusk_horn
REDIS_URL=redis://localhost:6379
```

## Features

### Implemented (Frontend)
- [x] Landing page with tribe showcase
- [x] Login/Register with Google OAuth
- [x] Onboarding (tribe selection)
- [x] Village view with building grid
- [x] Resource bar with production rates
- [x] Building detail modal
- [x] Build menu modal
- [x] World map with navigation
- [x] Tile detail modal

### In Progress
- [ ] Backend API endpoints
- [ ] Real-time resource updates
- [ ] Troop training
- [ ] Army movement
- [ ] Combat system
- [ ] Alliance system

## Game Mechanics

### Resources
- Wood, Clay, Iron, Crop
- Production based on building levels
- Storage capacity limits

### Buildings (23 types)
- **Infrastructure:** Main Building, Warehouse, Granary, Market, Cranny
- **Military:** Barracks, Stable, Workshop, Academy, Smithy, Wall, Rally Point
- **Special:** Embassy, Palace, Residence, Hero Mansion, Tavern
- **Resource:** Woodcutter, Clay Pit, Iron Mine, Crop Field

### Troops (16 types)
Each tribe has unique units with different stats for attack, defense, speed, and carry capacity.

### Village Types
Different resource distributions:
- Balanced (4-4-4-6)
- Cropper (1-1-1-15)
- 9-Cropper (3-3-3-9)
- Resource Heavy (5-4-5-4)

## Development

### Commands

```bash
# Frontend
cd frontend
npm run dev          # Start dev server
npm run build        # Production build
npm run preview      # Preview production build

# Backend
cd backend
cargo run            # Start server
cargo test           # Run tests
cargo build --release  # Production build

# Database
sqlx migrate run     # Run migrations
sqlx migrate revert  # Rollback migration
```

## Documentation

- [Specification](.docs/01-spec.md) - Game design document
- [Technical Plan](.docs/02-pland.md) - Architecture and implementation plan
- [Tasks](.docs/03-task.md) - Development task list

## License

MIT
