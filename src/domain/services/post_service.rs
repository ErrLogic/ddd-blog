use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    models::post::{CreatePost, Post, UpdatePost},
    repositories::PostRepository,
};
use crate::shared::error::ApiError;

#[async_trait]
pub trait PostService: Send + Sync {
    async fn get_post(&self, id: Uuid) -> Result<Post, ApiError>;
    async fn get_posts(&self) -> Result<Vec<Post>, ApiError>;
    async fn create_post(&self, post: CreatePost) -> Result<Post, ApiError>;
    async fn update_post(&self, id: Uuid, post: UpdatePost) -> Result<Post, ApiError>;
    async fn delete_post(&self, id: Uuid) -> Result<(), ApiError>;
}

#[derive(Clone)]
pub struct PostServiceImpl<R: PostRepository + Send + Sync + 'static> {
    repository: Arc<R>,
}

impl<R: PostRepository + Send + Sync + 'static> PostServiceImpl<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R: PostRepository + Send + Sync + 'static> PostService for Arc<PostServiceImpl<R>> {
    async fn get_post(&self, id: Uuid) -> Result<Post, ApiError> {
        self.repository.find(id).await
    }

    async fn get_posts(&self) -> Result<Vec<Post>, ApiError> {
        self.repository.find_all().await
    }

    async fn create_post(&self, post: CreatePost) -> Result<Post, ApiError> {
        let new_post = Post {
            id: Uuid::new_v4(),
            title: post.title,
            content: post.content,
            author_id: post.author_id,
            published: false,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        };

        self.repository.create(new_post).await
    }

    async fn update_post(&self, id: Uuid, post: UpdatePost) -> Result<Post, ApiError> {
        let mut existing_post = self.repository.find(id).await?;

        if let Some(title) = post.title {
            existing_post.title = title;
        }

        if let Some(content) = post.content {
            existing_post.content = content;
        }

        if let Some(published) = post.published {
            existing_post.published = published;
        }

        existing_post.updated_at = chrono::Local::now().naive_local();

        self.repository.update(id, existing_post).await
    }

    async fn delete_post(&self, id: Uuid) -> Result<(), ApiError> {
        self.repository.delete(id).await
    }
}
