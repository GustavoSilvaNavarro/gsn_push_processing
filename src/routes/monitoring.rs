use actix_web::{HttpResponse, Responder, get, web};

#[get("/healthz")]
async fn get_health_check() -> impl Responder {
    HttpResponse::NoContent().finish()
}

#[get("/checkz")]
async fn check_service() -> impl Responder {
    HttpResponse::Ok().body("Service Health checked ✅")
}

pub fn cfg_monitoring_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_health_check).service(check_service);
}
