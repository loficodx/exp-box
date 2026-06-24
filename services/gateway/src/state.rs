use sqlx::SqlitePool;

use crate::room_registry::RoomRegistry;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub http: reqwest::Client,
    pub auth_service_url: String,
    pub rooms: RoomRegistry,
}

impl AppState {
    pub fn new(pool: SqlitePool) -> Self {
        let auth_service_url = std::env::var("AUTH_SERVICE_URL")
            .unwrap_or_else(|_| "http://auth-service:9001".to_string());
        Self {
            pool,
            http: reqwest::Client::new(),
            auth_service_url,
            rooms: RoomRegistry::default(),
        }
    }
}
