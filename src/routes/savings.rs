use crate::models::transactions::CreateTransaction;
use crate::services::SavingsService;
use actix_web::{
    HttpResponse, Responder, post,
    web::{Data, Json, ServiceConfig},
};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[post("/new-saving")]
async fn add_new_saving_value(
    db: Data<PgPool>,
    payload: Json<CreateTransaction>,
) -> impl Responder {
    match SavingsService::create_new_saving(&db, &payload.into_inner()).await {
        Ok(transaction) => {
            log::info!("✅ Created transaction: id={}", transaction.id);
            HttpResponse::Created().json(transaction)
        }
        Err(e) => {
            log::error!("❌ Failed to create transaction: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("Failed to create transaction: {}", e),
            })
        }
    }
}

pub fn cfg_savings_routes(cfg: &mut ServiceConfig) {
    cfg.service(add_new_saving_value);
}
