use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::models::comment::{CreateComment, UpdateComment};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentResponse {
    pub id: Uuid,
    pub content: String,
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<crate::domain::models::comment::Comment> for CommentResponse {
    fn from(comment: crate::domain::models::comment::Comment) -> Self {
        Self {
            id: comment.id,
            content: comment.content,
            post_id: comment.post_id,
            author_id: comment.author_id,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCommentRequest {
    #[validate(length(min = 1))]
    pub content: String,
    pub post_id: Uuid,
}

impl From<CreateCommentRequest> for CreateComment {
    fn from(request: CreateCommentRequest) -> Self {
        Self {
            content: request.content,
            post_id: request.post_id,
            author_id: Uuid::new_v4(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateCommentRequest {
    #[validate(length(min = 1))]
    pub content: Option<String>,
}

impl From<UpdateCommentRequest> for UpdateComment {
    fn from(request: UpdateCommentRequest) -> Self {
        Self {
            content: request.content,
        }
    }
}
