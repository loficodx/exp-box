use anyhow::Result;

mod api;
mod app;
mod db;
mod error;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    let state = state::AppState::new();
    let app = app::build_app(state)?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9001").await?;

    println!("Listening on http://0.0.0.0:9001");

    axum::serve(listener, app).await?;

    Ok(())
}
