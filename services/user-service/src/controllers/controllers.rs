use rocket::{State, get, post, routes, serde::json::Json};
use shared_auth::guards::AuthenticatedUser;
use shared_common::responses::ApiResponse;

use crate::{
    models::{CreateUserRequest, LoginRequest, LoginResponse, UserResponse},
    service::UserService,
};

// #[post("/register", data = "<request>")]
// pub async fn register(
//     user_service: &State<UserService>,
//     request: Json<CreateUserRequest>,
// ) -> ApiResult<Json<ApiResponse<UserResponse>>> {
//     let result = user_service.register_user(request.into_inner()).await;

//     match result {
//         Ok(user) => Ok(Json(ApiResponse::success(user))),
//         Err(e) => Ok(Json(ApiResponse::error(e.to_string()))),
//     }
// }

#[post("/register", data = "<request>")]
pub async fn register(
    user_state: &State<UserService>,
    request: Json<CreateUserRequest>,
) -> Json<ApiResponse<UserResponse>> {
    let result = user_state.register_user(request.into_inner()).await;

    Json(
        result
            .map(ApiResponse::success)
            .unwrap_or_else(|e| ApiResponse::error(e.to_string())),
    )
}

// #[post("/login", data = "<request>")]
// pub async fn login(
//     user_service: &State<UserService>,
//     request: Json<LoginRequest>,
// ) -> ApiResult<Json<ApiResponse<LoginResponse>>> {
//     let result = user_service.login(request.into_inner()).await;

//     match result {
//         Ok(response) => Ok(Json(ApiResponse::success(response))),
//         Err(e) => Ok(Json(ApiResponse::error(e.to_string()))),
//     }
// }

#[post("/login", data = "<request>")]
pub async fn login(
    user_service: &State<UserService>,
    request: Json<LoginRequest>,
) -> Json<ApiResponse<LoginResponse>> {
    let result = user_service.login(request.into_inner()).await;

    let return_value = result
        .map(ApiResponse::success)
        .unwrap_or_else(|e| ApiResponse::error(e.to_string()));

    Json(return_value)
}

// #[get("/profile")]
// pub async fn get_profile(user_service: &State<UserService>, user: Auth)

#[get("/profile")]
pub async fn get_profile(
    user_service: &State<UserService>,
    user: AuthenticatedUser,
) -> Json<ApiResponse<UserResponse>> {
    let result = user_service.get_user_profile(user.id).await;

    Json(
        result
            .map(ApiResponse::success)
            .unwrap_or_else(|e| ApiResponse::error(e.to_string())),
    )
    // unimplemented!()
}

pub fn routes() -> Vec<rocket::Route> {
    routes![register, login, get_profile]
}
