use actix_web::{HttpResponse, Responder, post, web};
use serde::Serialize;

#[derive(Serialize)]
struct Msg {
    msg: String,
}

#[post("/new-saving")]
async fn add_new_saving_value() -> impl Responder {
    let resp = Msg {
        msg: "Need to add new saving metric".to_string(),
    };
    HttpResponse::Created().json(resp)
}

pub fn cfg_savings_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(add_new_saving_value);
}
