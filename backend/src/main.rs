use anyhow::Result;

mod api;
mod app;
mod db;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = db::init_pool().await?;
    let state = state::AppState::new(pool);
    let app = app::build_app(state)?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;

    println!("Listening on http://0.0.0.0:8000");

    axum::serve(listener, app).await?;

    Ok(())
}
