use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    models::comment::{Comment, CreateComment, UpdateComment},
    repositories::CommentRepository,
};
use crate::shared::error::ApiError;

#[async_trait]
pub trait CommentService: Send + Sync {
    async fn find(&self, id: Uuid) -> Result<Comment, ApiError>;
    async fn find_by_post(&self, post_id: Uuid) -> Result<Vec<Comment>, ApiError>;
    async fn create(&self, comment: CreateComment) -> Result<Comment, ApiError>;
    async fn update(&self, id: Uuid, comment: UpdateComment) -> Result<Comment, ApiError>;
    async fn delete(&self, id: Uuid) -> Result<(), ApiError>;
}

#[derive(Clone)]
pub struct CommentServiceImpl<R: CommentRepository + Send + Sync + 'static> {
    repository: Arc<R>,
}

impl<R: CommentRepository + Send + Sync + 'static> CommentServiceImpl<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: CommentRepository + Send + Sync + 'static> CommentService for Arc<CommentServiceImpl<R>> {
    async fn find(&self, id: Uuid) -> Result<Comment, ApiError> {
        self.repository.find(id).await
    }

    async fn find_by_post(&self, post_id: Uuid) -> Result<Vec<Comment>, ApiError> {
        self.repository.find_by_post(post_id).await
    }

    async fn create(&self, comment: CreateComment) -> Result<Comment, ApiError> {
        let new_comment = Comment {
            id: Uuid::new_v4(),
            content: comment.content,
            post_id: comment.post_id,
            author_id: comment.author_id,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        };

        self.repository.create(new_comment).await
    }

    async fn update(&self, id: Uuid, comment: UpdateComment) -> Result<Comment, ApiError> {
        let mut existing_comment = self.repository.find(id).await?;

        if let Some(content) = comment.content {
            existing_comment.content = content;
        }

        existing_comment.updated_at = chrono::Local::now().naive_local();

        self.repository.update(id, existing_comment).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), ApiError> {
        self.repository.delete(id).await
    }
}
