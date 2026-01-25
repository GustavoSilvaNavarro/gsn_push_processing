use crate::errors::{AppError, AppResult};
use crate::models::transactions::{CreateTransaction, Transaction, UpdateTransaction};
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

    // List all transactions
    pub async fn list_savings(db: &PgPool, limit: i32, offset: i32) -> AppResult<Vec<Transaction>> {
        sqlx::query_as::<_, Transaction>(
            r#"
            SELECT id, amount, source, created_at, updated_at
            FROM transactions
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(db)
        .await
        .map_err(AppError::from)
    }

    // Updates
    pub async fn update_saving(
        db: &PgPool,
        saving_id: i64,
        payload: &UpdateTransaction,
    ) -> AppResult<Transaction> {
        if payload.amount.is_none() && payload.source.is_none() {
            return Err(AppError::BadRequest(
                "At least one field must be provided for update".to_string(),
            ));
        }

        let result = sqlx::query_as::<_, Transaction>(
            r#"
            UPDATE transactions
            SET
                amount = COALESCE($1, amount),
                source = COALESCE($2, source),
                updated_at = NOW()
            WHERE id = $3
            RETURNING id, amount, source, created_at, updated_at
            "#,
        )
        .bind(&payload.amount)
        .bind(&payload.source)
        .bind(saving_id)
        .fetch_optional(db)
        .await?;

        result.ok_or_else(|| AppError::NotFound(format!("Saving with ID {} not found", saving_id)))
    }

    pub async fn delete_saving(db: &PgPool, saving_id: i64) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM transactions WHERE id = $1")
            .bind(saving_id)
            .execute(db)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!(
                "Saving with ID {} not found",
                saving_id
            )));
        }

        Ok(())
    }
}
