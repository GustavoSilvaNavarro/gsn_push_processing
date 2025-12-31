mod config;
mod adapters;

use config::Config;
use actix_web::{HttpResponse, Responder, get, App, HttpServer};
use crate::adapters::logger;

#[get("/healthz")]
async fn healthz() -> impl Responder {
  HttpResponse::NoContent().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let config = Config::from_env();
  logger::init_logger(&config);

  let server = HttpServer::new(|| {
    App::new().service(healthz)
  });


  log::info!("🚀 Application: {} running on port {}", config.name, config.port);
  server.bind((config.app_host, config.port))?.run().await
}
