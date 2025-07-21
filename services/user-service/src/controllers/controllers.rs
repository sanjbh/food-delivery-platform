use rocket::{State, get, post, serde::json::Json};
use shared_common::responses::{ApiResponse, ApiResult};

use crate::{
    models::{CreateUserRequest, LoginRequest, LoginResponse, UserResponse},
    service::UserService,
};

#[post("/register", data = "<request>")]
pub async fn register(
    user_service: &State<UserService>,
    request: Json<CreateUserRequest>,
) -> ApiResult<Json<ApiResponse<UserResponse>>> {
    let result = user_service.register_user(request.into_inner()).await;

    match result {
        Ok(user) => Ok(Json(ApiResponse::success(user))),
        Err(e) => Ok(Json(ApiResponse::error(e.to_string()))),
    }
}

#[post("/login", data = "<request>")]
pub async fn login(
    user_service: &State<UserService>,
    request: Json<LoginRequest>,
) -> ApiResult<Json<ApiResponse<LoginResponse>>> {
    let result = user_service.login(request.into_inner()).await;

    match result {
        Ok(response) => Ok(Json(ApiResponse::success(response))),
        Err(e) => Ok(Json(ApiResponse::error(e.to_string()))),
    }
}

// #[get("/profile")]
// pub async fn get_profile(user_service: &State<UserService>, user: Auth)
