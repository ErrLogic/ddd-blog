use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    models::user::{CreateUser, UpdateUser, User},
    repositories::UserRepository,
};
use crate::shared::error::ApiError;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn find(&self, id: Uuid) -> Result<User, ApiError>;
    async fn find_all(&self) -> Result<Vec<User>, ApiError>;
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
    async fn find_all(&self) -> Result<Vec<User>, ApiError> {
        self.repository.find_all().await
    }

    async fn find(&self, id: Uuid) -> Result<User, ApiError> {
        self.repository.find(id).await
    }

    async fn find_by_email(&self, email: &str) -> Result<User, ApiError> {
        self.repository.find_by_email(email).await
    }

    async fn create(&self, user: CreateUser) -> Result<User, ApiError> {
        let new_user = User::new(user.username, user.email, user.password)?;
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
            existing_user.update_password(password)?;
        }

        existing_user.updated_at = chrono::Local::now().naive_local();
        self.repository.update(id, existing_user).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), ApiError> {
        self.repository.delete(id).await
    }
}
