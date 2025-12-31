mod config;
mod adapters;

use config::Config;
use actix_web::{HttpResponse, Responder, get, App, HttpServer, middleware::{Logger, Compress}};
use crate::adapters::logger;

#[get("/healthz")]
async fn healthz() -> impl Responder {
  HttpResponse::NoContent().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let config = Config::from_env();
  logger::init_logger(&config);

  let bind_address = (config.app_host.as_str(), config.port);
  let workers = num_cpus::get().clamp(1, 4);

  let server = HttpServer::new(|| {
    App::new().wrap(Logger::default()).wrap(Compress::default()).service(healthz)
  });

  log::info!("🚀 Application: {} running on {}:{}", config.name, config.app_host, config.port);
  server.workers(workers).bind(bind_address)?.run().await
}
