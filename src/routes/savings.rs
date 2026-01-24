use crate::errors::{AppError, AppResult};
use crate::models::transactions::CreateTransaction;
use crate::services::SavingsService;
use actix_web::{
    HttpResponse, get, post,
    web::{Data, Json, Path, ServiceConfig},
};
use sqlx::PgPool;
use validator::Validate;

#[post("/new-saving")]
async fn add_new_saving_value(
    db: Data<PgPool>,
    payload: Json<CreateTransaction>,
) -> AppResult<HttpResponse> {
    payload.validate()?;
    let transaction = SavingsService::create_new_saving(&db, &payload.into_inner()).await?;
    Ok(HttpResponse::Created().json(transaction))
}

#[get("/savings/{saving_id}")]
async fn get_saving_by_id(db: Data<PgPool>, saving_id: Path<i64>) -> AppResult<HttpResponse> {
    if *saving_id <= 0 {
        return Err(AppError::BadRequest(
            "Invalid ID: must be a positive integer".to_string(),
        ));
    }
    let transaction = SavingsService::get_by_id(&db, *saving_id).await?;

    match transaction {
        Some(t) => Ok(HttpResponse::Ok().json(t)),
        None => Err(AppError::NotFound("Saving not found".to_string())),
    }
}

pub fn cfg_savings_routes(cfg: &mut ServiceConfig) {
    cfg.service(add_new_saving_value).service(get_saving_by_id);
}
