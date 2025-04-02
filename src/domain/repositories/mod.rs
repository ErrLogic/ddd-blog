use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::models::{comment::Comment, post::Post, user::User};
use crate::shared::error::ApiError;

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn find(&self, id: Uuid) -> Result<Post, ApiError>;
    async fn find_all(&self) -> Result<Vec<Post>, ApiError>;
    async fn create(&self, post: Post) -> Result<Post, ApiError>;
    async fn update(&self, id: Uuid, post: Post) -> Result<Post, ApiError>;
    async fn delete(&self, id: Uuid) -> Result<(), ApiError>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find(&self, id: Uuid) -> Result<User, ApiError>;
    async fn find_all(&self) -> Result<Vec<User>, ApiError>;
    async fn find_by_email(&self, email: &str) -> Result<User, ApiError>;
    async fn create(&self, user: User) -> Result<User, ApiError>;
    async fn update(&self, id: Uuid, user: User) -> Result<User, ApiError>;
    async fn delete(&self, id: Uuid) -> Result<(), ApiError>;
}

#[async_trait]
pub trait CommentRepository: Send + Sync {
    async fn find(&self, id: Uuid) -> Result<Comment, ApiError>;
    async fn find_by_post(&self, post_id: Uuid) -> Result<Vec<Comment>, ApiError>;
    async fn create(&self, comment: Comment) -> Result<Comment, ApiError>;
    async fn update(&self, id: Uuid, comment: Comment) -> Result<Comment, ApiError>;
    async fn delete(&self, id: Uuid) -> Result<(), ApiError>;
}
