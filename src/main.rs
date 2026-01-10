mod adapters;
mod config;
mod errors;
mod models;
mod routes;
mod services;

use actix_web::{
    App, HttpServer,
    error::{InternalError, JsonPayloadError},
    middleware::{Compress, Logger},
    web::{Data, JsonConfig, scope},
};
use adapters::{db, logger};
use config::Config;

fn json_error_handler(err: JsonPayloadError, _req: &actix_web::HttpRequest) -> actix_web::Error {
    let error_message = match &err {
        JsonPayloadError::Deserialize(de_err) => de_err.to_string(),
        JsonPayloadError::ContentType => {
            "Invalid content type, expected application/json".to_string()
        }
        JsonPayloadError::Payload(payload_err) => format!("Payload error: {}", payload_err),
        _ => "Invalid JSON payload".to_string(),
    };

    let response = serde_json::json!({
        "error": error_message
    });

    InternalError::from_response(err, actix_web::HttpResponse::BadRequest().json(response)).into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    logger::init_logger(&config);

    // Initialize database connection pool
    let db_config = db::DatabaseConfig {
        url: config.database_url.clone(),
        max_connections: config.db_max_connections,
        min_connections: config.db_min_connections,
        connect_timeout: 30,
        idle_timeout: 600,
    };

    let pool = db::init_pool(&db_config)
        .await
        .expect("Failed to initialize database pool");

    // Database health check
    if let Err(e) = db::health_check(&pool).await {
        log::error!("❌ Database health check failed: {}", e);
        panic!("Database is not healthy");
    }

    let bind_address = (config.app_host.as_str(), config.port);
    let workers = num_cpus::get().clamp(1, 4);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(JsonConfig::default().error_handler(json_error_handler))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .configure(routes::cfg_monitoring_routes)
            .service(scope(&config.url_prefix).configure(routes::cfg_savings_routes))
    });

    log::info!(
        "🚀 Application: {} running on {}:{}",
        config.name,
        config.app_host,
        config.port
    );
    server.workers(workers).bind(bind_address)?.run().await
}
