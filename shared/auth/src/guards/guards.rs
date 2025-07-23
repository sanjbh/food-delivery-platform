use rocket::{
    Request,
    http::Status,
    request::{FromRequest, Outcome},
};
use uuid::Uuid;

use crate::jwt::verify_access_token;

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub email: String,
    pub user_type: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = &'static str;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = match request.headers().get_one("Authorization") {
            Some(header) => header,
            None => {
                return Outcome::Error((Status::Unauthorized, "Authorization header not present"));
            }
        };

        if !auth_header.starts_with("Bearer ") {
            return Outcome::Error((Status::Unauthorized, "Invalid authorization format"));
        }

        let token = &auth_header[7..];

        match verify_access_token(token) {
            Ok(claim) => {
                let id = match Uuid::parse_str(&claim.sub) {
                    Ok(id) => id,
                    Err(e) => return Outcome::Error((Status::Unauthorized, "Invalid user id")),
                };
                Outcome::Success(AuthenticatedUser {
                    id,
                    email: claim.email,
                    user_type: claim.user_type,
                })
            }
            Err(_) => return Outcome::Error((Status::Unauthorized, "Invalid token")),
        }
    }
}
