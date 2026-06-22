use anyhow::{Context, Result};
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::path::Path;

const DEFAULT_DATABASE_URL: &str = "sqlite://data/exp-box.db";

pub async fn init_pool() -> Result<SqlitePool> {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_string());

    ensure_sqlite_dir(&database_url)?;

    let connect_opts: SqliteConnectOptions =
        database_url.parse().context("invalid DATABASE_URL")?;

    let pool = SqlitePool::connect_with(connect_opts.create_if_missing(true))
        .await
        .context("failed to connect to SQLite database")?;

    sqlx::migrate!("../migrations")
        .run(&pool)
        .await
        .context("failed to run database migrations")?;

    Ok(pool)
}

fn ensure_sqlite_dir(database_url: &str) -> Result<()> {
    let Some(db_path) = database_url.strip_prefix("sqlite://") else {
        return Ok(());
    };

    if let Some(dir) = Path::new(db_path)
        .parent()
        .filter(|dir| !dir.as_os_str().is_empty())
    {
        std::fs::create_dir_all(dir).with_context(|| {
            format!(
                "failed to create SQLite database directory: {}",
                dir.display()
            )
        })?;
    }

    Ok(())
}
