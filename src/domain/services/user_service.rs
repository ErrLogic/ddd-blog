use std::sync::Arc;

use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use async_trait::async_trait;
use rand_core::OsRng;
use uuid::Uuid;

use crate::domain::{
    models::user::{CreateUser, UpdateUser, User},
    repositories::UserRepository,
};
use crate::shared::error::ApiError;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn find(&self, id: Uuid) -> Result<User, ApiError>;
    async fn find_by_email(&self, email: &str) -> Result<User, ApiError>;
    async fn create(&self, user: CreateUser) -> Result<User, ApiError>;
    async fn update(&self, id: Uuid, user: UpdateUser) -> Result<User, ApiError>;
    async fn delete(&self, id: Uuid) -> Result<(), ApiError>;
}

#[derive(Clone)]
pub struct UserServiceImpl<R: UserRepository + Send + Sync + 'static> {
    repository: Arc<R>,
}

impl<R: UserRepository + Send + Sync + 'static> UserServiceImpl<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: UserRepository + Send + Sync + 'static> UserService for Arc<UserServiceImpl<R>> {
    async fn find(&self, id: Uuid) -> Result<User, ApiError> {
        self.repository.find(id).await
    }

    async fn find_by_email(&self, email: &str) -> Result<User, ApiError> {
        self.repository.find_by_email(email).await
    }

    async fn create(&self, user: CreateUser) -> Result<User, ApiError> {
        // Generate a random salt
        let salt = SaltString::generate(&mut OsRng);

        // Configure Argon2 with default parameters
        let argon2 = Argon2::default();

        // Hash the password
        let password_hash = argon2
            .hash_password(user.password.as_bytes(), &salt)
            .map_err(|_e| ApiError::InternalServerError)?
            .to_string();

        let new_user = User {
            id: Uuid::new_v4(),
            username: user.username,
            email: user.email,
            password_hash: password_hash,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        };

        self.repository.create(new_user).await
    }

    async fn update(&self, id: Uuid, user: UpdateUser) -> Result<User, ApiError> {
        let mut existing_user = self.repository.find(id).await?;

        if let Some(username) = user.username {
            existing_user.username = username;
        }

        if let Some(email) = user.email {
            existing_user.email = email;
        }

        if let Some(password) = user.password {
            // Generate a random salt
            let salt = SaltString::generate(&mut OsRng);

            // Configure Argon2 with default parameters
            let argon2 = Argon2::default();

            // Hash the password - we already have it unwrapped in this block
            let password_hash = argon2
                .hash_password(password.as_bytes(), &salt)
                .map_err(|_e| ApiError::InternalServerError)?
                .to_string();

            existing_user.password_hash = password_hash;
        }

        existing_user.updated_at = chrono::Local::now().naive_local();

        self.repository.update(id, existing_user).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), ApiError> {
        self.repository.delete(id).await
    }
}
