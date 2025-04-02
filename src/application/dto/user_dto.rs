use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::models::user::{CreateUser, UpdateUser};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<crate::domain::models::user::User> for UserResponse {
    fn from(user: crate::domain::models::user::User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

impl From<CreateUserRequest> for CreateUser {
    fn from(request: CreateUserRequest) -> Self {
        Self {
            username: request.username,
            email: request.email,
            password: request.password,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 8))]
    pub password: Option<String>,
}

impl From<UpdateUserRequest> for UpdateUser {
    fn from(request: UpdateUserRequest) -> Self {
        Self {
            username: request.username,
            email: request.email,
            password: request.password,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUserResponse {
    pub user: UserResponse,
    pub token: String,
}
