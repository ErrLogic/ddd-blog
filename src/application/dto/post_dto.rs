use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::models::post::{CreatePost, UpdatePost};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub author_id: Uuid,
    pub published: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<crate::domain::models::post::Post> for PostResponse {
    fn from(post: crate::domain::models::post::Post) -> Self {
        Self {
            id: post.id,
            title: post.title,
            content: post.content,
            author_id: post.author_id,
            published: post.published,
            created_at: post.created_at,
            updated_at: post.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePostRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
}

impl From<CreatePostRequest> for CreatePost {
    fn from(request: CreatePostRequest) -> Self {
        Self {
            title: request.title,
            content: request.content,
            author_id: Uuid::new_v4(), // In real app, this would come from auth
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePostRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: Option<String>,
    #[validate(length(min = 1))]
    pub content: Option<String>,
    pub published: Option<bool>,
}

impl From<UpdatePostRequest> for UpdatePost {
    fn from(request: UpdatePostRequest) -> Self {
        Self {
            title: request.title,
            content: request.content,
            published: request.published,
        }
    }
}
