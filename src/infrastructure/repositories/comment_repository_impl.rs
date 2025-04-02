use std::sync::Arc;

use async_trait::async_trait;
use diesel::{QueryDsl, RunQueryDsl, prelude::*};
use uuid::Uuid;

use crate::{
    domain::models::comment::{Comment, NewComment, UpdateCommentData},
    domain::repositories::CommentRepository,
    infrastructure::database::connection::PgPool,
    shared::error::ApiError,
};

#[derive(Clone)]
pub struct CommentRepositoryImpl {
    pool: PgPool,
}

impl CommentRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CommentRepository for CommentRepositoryImpl {
    async fn find(&self, _id: Uuid) -> Result<Comment, ApiError> {
        use crate::infrastructure::database::schema::comments::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        comments
            .filter(id.eq(id))
            .select(Comment::as_select())
            .first(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    async fn find_by_post(&self, _post_id: Uuid) -> Result<Vec<Comment>, ApiError> {
        use crate::infrastructure::database::schema::comments::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        comments
            .filter(post_id.eq(post_id))
            .select(Comment::as_select())
            .load(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    async fn create(&self, comment: Comment) -> Result<Comment, ApiError> {
        use crate::infrastructure::database::schema::comments::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        let new_comment = NewComment {
            content: comment.content,
            post_id: comment.post_id,
            author_id: comment.author_id,
        };

        diesel::insert_into(comments)
            .values(&new_comment)
            .returning(Comment::as_returning())
            .get_result(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    async fn update(&self, comment_id: Uuid, comment: Comment) -> Result<Comment, ApiError> {
        use crate::infrastructure::database::schema::comments::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        let update_data = UpdateCommentData {
            content: Some(comment.content),
            updated_at: Some(chrono::Local::now().naive_local()),
        };

        diesel::update(comments.filter(id.eq(comment_id)))
            .set(&update_data)
            .returning(Comment::as_returning())
            .get_result(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    async fn delete(&self, comment_id: Uuid) -> Result<(), ApiError> {
        use crate::infrastructure::database::schema::comments::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        diesel::delete(comments.filter(id.eq(comment_id)))
            .execute(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl CommentRepository for Arc<CommentRepositoryImpl> {
    async fn find(&self, id: Uuid) -> Result<Comment, ApiError> {
        self.as_ref().find(id).await
    }

    async fn find_by_post(&self, post_id: Uuid) -> Result<Vec<Comment>, ApiError> {
        self.as_ref().find_by_post(post_id).await
    }

    async fn create(&self, comment: Comment) -> Result<Comment, ApiError> {
        self.as_ref().create(comment).await
    }

    async fn update(&self, comment_id: Uuid, comment: Comment) -> Result<Comment, ApiError> {
        self.as_ref().update(comment_id, comment).await
    }

    async fn delete(&self, comment_id: Uuid) -> Result<(), ApiError> {
        self.as_ref().delete(comment_id).await
    }
}
