use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{infrastructure::database::schema::users, shared::error::ApiError};

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUserData {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
    #[diesel(column_name = "updated_at")]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateUser {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateUser {
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 8))]
    pub password: Option<String>,
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Result<Self, ApiError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| ApiError::InternalServerError)?
            .to_string();

        Ok(Self {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        })
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, ApiError> {
        let parsed_hash =
            PasswordHash::new(&self.password_hash).map_err(|_| ApiError::InternalServerError)?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    pub fn update_password(&mut self, new_password: String) -> Result<(), ApiError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        self.password_hash = argon2
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|_| ApiError::InternalServerError)?
            .to_string();
        self.updated_at = chrono::Local::now().naive_local();
        Ok(())
    }
}
