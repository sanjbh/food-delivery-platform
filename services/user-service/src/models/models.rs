use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, prelude::Type};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub phone: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub user_type: UserType,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "user_type", rename_all = "lowercase")]
pub enum UserType {
    Customer,
    RestaurantOwner,
    DeliveryDriver,
    Admin,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 10, max = 15))]
    pub phone: String,

    #[validate(length(min = 2, max = 50))]
    pub first_name: String,

    #[validate(length(min = 2, max = 50))]
    pub last_name: String,

    #[validate(length(min = 8))]
    pub password: String,

    pub user_type: UserType,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,

    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub phone: String,
    pub first_name: String,
    pub last_name: String,
    pub user_type: UserType,
    pub is_verified: bool,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: UserResponse,
    pub access_token: String,
    pub refresh_token: String,
}
