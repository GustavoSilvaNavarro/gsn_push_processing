use crate::errors::AppResult;
use crate::models::transactions::CreateTransaction;
use crate::services::SavingsService;
use actix_web::{
    HttpResponse, post,
    web::{Data, Json, ServiceConfig},
};
use serde::Serialize;
use sqlx::PgPool;
use validator::Validate;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
struct ValidationErrorResponse {
    error: String,
    details: Vec<String>,
}

#[post("/new-saving")]
async fn add_new_saving_value(
    db: Data<PgPool>,
    payload: Json<CreateTransaction>,
) -> AppResult<HttpResponse> {
    payload.validate()?;
    let transaction = SavingsService::create_new_saving(&db, &payload.into_inner()).await?;
    Ok(HttpResponse::Created().json(transaction))
}

pub fn cfg_savings_routes(cfg: &mut ServiceConfig) {
    cfg.service(add_new_saving_value);
}
