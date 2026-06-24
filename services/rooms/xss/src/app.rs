use anyhow::Result;
use axum::Router;

use crate::{api, state::AppState};

pub fn build_app(state: AppState) -> Result<Router> {
    let app = Router::new().merge(api::router()).with_state(state);
    Ok(app)
}
