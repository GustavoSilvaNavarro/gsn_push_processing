mod adapters;
mod config;
mod routes;

use actix_web::{
    App, HttpServer,
    middleware::{Compress, Logger},
};
use adapters::logger;
use config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    logger::init_logger(&config);

    let bind_address = (config.app_host.as_str(), config.port);
    let workers = num_cpus::get().clamp(1, 4);

    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Compress::default())
            .configure(routes::cfg_monitoring_routes)
    });

    log::info!(
        "🚀 Application: {} running on {}:{}",
        config.name,
        config.app_host,
        config.port
    );
    server.workers(workers).bind(bind_address)?.run().await
}
