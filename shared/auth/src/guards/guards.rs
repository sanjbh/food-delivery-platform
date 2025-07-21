use rocket::{
    Request,
    request::{FromRequest, Outcome},
    tokio,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub email: String,
    pub user_type: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = &'static str;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {}
}
