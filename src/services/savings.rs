use crate::errors::{AppError, AppResult};
use crate::models::transactions::{CreateTransaction, Transaction};
use sqlx::PgPool;

pub struct SavingsService;

impl SavingsService {
    pub async fn create_new_saving(
        db: &PgPool,
        payload: &CreateTransaction,
    ) -> AppResult<Transaction> {
        sqlx::query_as::<_, Transaction>(
            r#"
            INSERT INTO transactions (amount, source, created_at, updated_at)
            VALUES ($1, $2, NOW(), NOW())
            RETURNING id, amount, source, created_at, updated_at
            "#,
        )
        .bind(payload.amount)
        .bind(&payload.source)
        .fetch_one(db)
        .await
        .map_err(AppError::from)
    }

    pub async fn get_by_id(db: &PgPool, id: i64) -> AppResult<Option<Transaction>> {
        sqlx::query_as::<_, Transaction>(
            r#"
            SELECT id, amount, source, created_at, updated_at
            FROM transactions
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(db)
        .await
        .map_err(AppError::from)
    }

    // /// List all transactions
    // pub async fn list(
    //     pool: &PgPool,
    //     limit: i64,
    //     offset: i64,
    // ) -> Result<Vec<Transaction>, sqlx::Error> {
    //     sqlx::query_as::<_, Transaction>(
    //         r#"
    //         SELECT id, amount, source, created_at, updated_at
    //         FROM transactions
    //         ORDER BY created_at DESC
    //         LIMIT $1 OFFSET $2
    //         "#,
    //     )
    //     .bind(limit)
    //     .bind(offset)
    //     .fetch_all(pool)
    //     .await
    // }

    // /// Update a transaction
    // pub async fn update(
    //     pool: &PgPool,
    //     id: i64,
    //     amount: Option<rust_decimal::Decimal>,
    //     source: Option<String>,
    // ) -> Result<Option<Transaction>, sqlx::Error> {
    //     // Build dynamic query based on what's being updated
    //     let mut query = String::from("UPDATE transactions SET updated_at = NOW()");
    //     let mut bind_count = 1;

    //     if amount.is_some() {
    //         query.push_str(&format!(", amount = ${}", bind_count));
    //         bind_count += 1;
    //     }
    //     if source.is_some() {
    //         query.push_str(&format!(", source = ${}", bind_count));
    //         bind_count += 1;
    //     }

    //     query.push_str(&format!(" WHERE id = ${} RETURNING id, amount, source, created_at, updated_at", bind_count));

    //     let mut q = sqlx::query_as::<_, Transaction>(&query);

    //     if let Some(amt) = amount {
    //         q = q.bind(amt);
    //     }
    //     if let Some(src) = source {
    //         q = q.bind(src);
    //     }

    //     q.bind(id).fetch_optional(pool).await
    // }

    pub async fn delete(db: &PgPool, id: i64) -> AppResult<bool> {
        let result = sqlx::query("DELETE FROM transactions WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
