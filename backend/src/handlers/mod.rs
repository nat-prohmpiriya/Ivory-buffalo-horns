mod auth;
mod building;
mod village;

use axum::{middleware, routing::{delete, get, post, put}, Router};

use crate::middleware::auth_middleware;
use crate::AppState;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/auth", auth_routes(state.clone()))
        .nest("/villages", village_routes(state.clone()))
        .nest("/map", map_routes(state.clone()))
}

fn auth_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/me", get(auth::me))
        .route("/sync", post(auth::sync_user))
        .route("/profile", put(auth::update_profile))
        .route("/account", delete(auth::delete_account))
        .route("/logout", delete(auth::logout))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}

fn village_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(village::list_villages))
        .route("/", post(village::create_village))
        .route("/{id}", get(village::get_village))
        .route("/{id}", put(village::update_village))
        // Building routes nested under village
        .route("/{village_id}/buildings", get(building::list_buildings))
        .route("/{village_id}/buildings/queue", get(building::get_build_queue))
        .route("/{village_id}/buildings/{slot}", post(building::build))
        .route("/{village_id}/buildings/{slot}/upgrade", post(building::upgrade))
        .route("/{village_id}/buildings/{slot}", delete(building::demolish))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}

fn map_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(village::get_map))
        .route_layer(middleware::from_fn_with_state(state, auth_middleware))
}
