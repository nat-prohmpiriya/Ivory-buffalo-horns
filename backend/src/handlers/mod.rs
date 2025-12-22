mod auth;

use axum::{middleware, routing::{delete, get, post, put}, Router};

use crate::middleware::auth_middleware;
use crate::AppState;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/auth", auth_routes(state))
    // .nest("/servers", server::routes())
    // .nest("/villages", village::routes())
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
