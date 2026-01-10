use sqlx::{Error, PgPool, postgres::PgPoolOptions};
use std::time::Duration;

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            max_connections: 10,
            min_connections: 2,
            connect_timeout: 10,
            idle_timeout: 600,
        }
    }
}

/// Initialize database connection pool
pub async fn init_pool(config: &DatabaseConfig) -> Result<PgPool, Error> {
    log::info!("🔌 Initializing database connection pool...");

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.connect_timeout))
        .idle_timeout(Duration::from_secs(config.idle_timeout))
        .connect(&config.url)
        .await?;

    log::info!("✅ Database connection pool initialized successfully");
    Ok(pool)
}

/// Run database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    log::info!("🔄 Running database migrations...");
    sqlx::migrate!("./migrations").run(pool).await?;

    log::info!("✅ Database migrations completed successfully");
    Ok(())
}

/// Health check for database connection
pub async fn health_check(pool: &PgPool) -> Result<(), Error> {
    sqlx::query("SELECT 1").fetch_one(pool).await?;
    Ok(())
}
