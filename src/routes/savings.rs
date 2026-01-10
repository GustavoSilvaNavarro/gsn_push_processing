use crate::models::transactions::CreateTransaction;
use crate::services::SavingsService;
use validator::Validate;
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

#[derive(Serialize)]
struct ValidationErrorResponse {
    error: String,
    details: Vec<String>,
}

#[post("/new-saving")]
async fn add_new_saving_value(
    db: Data<PgPool>,
    payload: Json<CreateTransaction>,
) -> impl Responder {
    // Validate the payload
    if let Err(validation_errors) = payload.validate() {
        let error_messages: Vec<String> = validation_errors
            .field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |error| {
                    format!(
                        "{}: {}",
                        field,
                        error.message.as_ref().unwrap_or(&"Invalid value".into())
                    )
                })
            })
            .collect();

        log::warn!("⚠️  Validation failed: {:?}", error_messages);

        return HttpResponse::BadRequest().json(ValidationErrorResponse {
            error: "Validation failed".to_string(),
            details: error_messages,
        });
    }

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
