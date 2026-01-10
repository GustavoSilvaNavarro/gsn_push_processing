use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: i64,
    pub amount: rust_decimal::Decimal,
    pub source: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Custom validator for Decimal amounts
fn validate_positive_amount(amount: &Decimal) -> Result<(), ValidationError> {
    if *amount <= Decimal::ZERO {
        return Err(ValidationError::new("amount_must_be_positive"));
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct CreateTransaction {
    #[validate(custom(
        function = "validate_positive_amount",
        message = "Amount must be greater than 0"
    ))]
    pub amount: Decimal,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Source must be between 1 and 255 characters"
    ))]
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct UpdateTransaction {
    #[validate(custom(
        function = "validate_positive_amount",
        message = "Amount must be greater than 0"
    ))]
    pub amount: Option<Decimal>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Source must be between 1 and 255 characters"
    ))]
    pub source: Option<String>,
}
