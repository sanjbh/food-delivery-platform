use bcrypt::{DEFAULT_COST, hash, verify};
use shared_auth::jwt::{create_access_token, create_refresh_token};
use shared_common::errors::AppError;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    models::{CreateUserRequest, LoginRequest, LoginResponse, UserResponse},
    repository::{Repository, UserRepository},
};

pub struct UserService {
    repository: Box<dyn Repository + Send + Sync>,
}

impl UserService {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            repository: Box::new(UserRepository::new(pool.clone())),
        }
    }

    pub async fn register_user(
        &self,
        request: CreateUserRequest,
    ) -> Result<UserResponse, AppError> {
        request
            .validate()
            .map_err(|err| AppError::ValidationError(err.to_string()))?;

        if let Some(_) = self.repository.find_by_email(&request.email).await? {
            return Err(AppError::ConflictError("User already exists".to_string()));
        }

        let password_hash = hash(&request.password, DEFAULT_COST)
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        let user = self.repository.create_user(&request, password_hash).await?;

        Ok(UserResponse {
            email: user.email,
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            is_verified: user.is_verified,
            phone: user.phone,
            user_type: user.user_type,
        })
    }

    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, AppError> {
        request
            .validate()
            .map_err(|err| AppError::ValidationError(err.to_string()))?;

        let user = self.repository.find_by_email(&request.email).await?.ok_or(
            AppError::UnauthorizedError("Invalid credentials".to_string()),
        )?;

        let password_valid = verify(&request.password, &user.password_hash).map_err(|e| {
            AppError::InternalError(format!("Password verification failed {}", e.to_string()))
        })?;

        if !password_valid {
            return Err(AppError::UnauthorizedError(
                "Invalid credentials".to_string(),
            ));
        }

        let access_token = create_access_token(user.id, &user.email).map_err(|e| {
            AppError::InternalError(format!("Cannot create access token: {}", e.to_string()))
        })?;

        let refresh_token = create_refresh_token(user.id).map_err(|e| {
            AppError::InternalError(format!("Cannot create refresh token: {}", e.to_string()))
        })?;

        Ok(LoginResponse {
            user: UserResponse {
                id: user.id,
                email: user.email,
                phone: user.phone,
                first_name: user.first_name,
                last_name: user.last_name,
                user_type: user.user_type,
                is_verified: user.is_verified,
            },
            access_token,
            refresh_token,
        })
    }

    pub async fn get_user_profile(&self, user_id: Uuid) -> Result<UserResponse, AppError> {
        let user = self
            .repository
            .find_by_id(user_id)
            .await?
            .ok_or(AppError::NotfoundError("User not found".to_string()))?;

        Ok(UserResponse {
            id: user.id,
            email: user.email,
            phone: user.phone,
            first_name: user.first_name,
            last_name: user.last_name,
            user_type: user.user_type,
            is_verified: user.is_verified,
        })
    }
}
