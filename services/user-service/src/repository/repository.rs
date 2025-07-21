use shared_common::errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CreateUserRequest, User, UserType};

pub struct UserRepository {
    pool: PgPool,
}

// #[async_trait::async_trait]
#[rocket::async_trait]
pub trait Repository: Send + Sync {
    async fn create_user(
        &self,
        request: &CreateUserRequest,
        password_hash: String,
    ) -> Result<User, AppError>;

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    async fn update_verification_status(&self, id: Uuid, verified: bool) -> Result<(), AppError>;

}

// #[async_trait::async_trait]
#[rocket::async_trait]
impl Repository for UserRepository {
    async fn create_user(
        &self,
        request: &CreateUserRequest,
        password_hash: String,
    ) -> Result<User, AppError> {
        let user_type = &request.user_type;
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, phone, first_name, last_name, password_hash, user_type)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, email, phone, first_name, last_name, password_hash, 
                     user_type as "user_type: UserType", is_verified as "is_verified!", created_at as "created_at!", updated_at as "updated_at!"
            "#,
            request.email,
            request.phone,
            request.first_name,
            request.last_name,
            password_hash,
            user_type as &UserType,
            
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(User, 
        r#"
            SELECT id, email, phone, first_name, last_name, password_hash,
                   user_type as "user_type: UserType", is_verified as "is_verified!", created_at as "created_at!", updated_at as "updated_at!"                   
            FROM users WHERE email = $1
        "#, email)
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(User, r#"
         SELECT id, email, phone, first_name, last_name, password_hash,
                   user_type as "user_type: UserType", is_verified as "is_verified!", created_at as "created_at!", updated_at as "updated_at!"
            FROM users WHERE id = $1
        "#, id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(user)
    }

    async fn update_verification_status(&self, id: Uuid, verified: bool) -> Result<(), AppError> {
        sqlx::query!("UPDATE users SET is_verified = $1, updated_at = NOW() WHERE id = $2", verified, id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
