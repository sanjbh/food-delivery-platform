use shared_common::errors::AppError;
use sqlx::PgPool;

use crate::models::{CreateUserRequest, User};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(
        &self,
        request: &CreateUserRequest,
        password_hash: String,
    ) -> Result<User, AppError> {
        /* let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, phone, first_name, last_name, password_hash, user_type)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, email, phone, first_name, last_name, password_hash, 
                     user_type as "user_type: UserType", is_verified, created_at, updated_at
            "#,
            request.email,
            request.phone,
            request.first_name,
            request.last_name,
            request.password_hash,
            request.user_type as UserType,
        )
        .fetch_one(&self.pool)
        .await?; */

        Err(AppError::InternalError("Test Error".to_string()))
    }
}
