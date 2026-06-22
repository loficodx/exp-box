use anyhow::Result;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};

use crate::{api, state::AppState};

pub fn build_app(state: AppState) -> Result<Router> {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost".parse::<axum::http::HeaderValue>()?)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api", api::router())
        .layer(cors)
        .with_state(state);

    Ok(app)
}
