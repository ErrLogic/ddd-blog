use std::sync::Arc;

use async_trait::async_trait;
use diesel::{QueryDsl, RunQueryDsl, prelude::*};
use uuid::Uuid;

use crate::{
    domain::models::post::{NewPost, Post, UpdatePostData},
    domain::repositories::PostRepository,
    infrastructure::database::connection::PgPool,
    shared::error::ApiError,
};

#[derive(Clone)]
pub struct PostRepositoryImpl {
    pool: PgPool,
}

impl PostRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostRepository for PostRepositoryImpl {
    async fn find(&self, _id: Uuid) -> Result<Post, ApiError> {
        use crate::infrastructure::database::schema::posts::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        posts
            .filter(id.eq(id))
            .select(Post::as_select())
            .first(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    async fn find_all(&self) -> Result<Vec<Post>, ApiError> {
        use crate::infrastructure::database::schema::posts::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        posts
            .select(Post::as_select())
            .load(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    async fn create(&self, post: Post) -> Result<Post, ApiError> {
        use crate::infrastructure::database::schema::posts::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        let new_post = NewPost {
            title: post.title,
            content: post.content,
            author_id: post.author_id,
            published: post.published,
        };

        diesel::insert_into(posts)
            .values(&new_post)
            .returning(Post::as_returning())
            .get_result(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    async fn update(&self, post_id: Uuid, post: Post) -> Result<Post, ApiError> {
        use crate::infrastructure::database::schema::posts::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        let update_data = UpdatePostData {
            title: Some(post.title),
            content: Some(post.content),
            published: Some(post.published),
            updated_at: Some(chrono::Local::now().naive_local()),
        };

        diesel::update(posts.filter(id.eq(post_id)))
            .set(&update_data)
            .returning(Post::as_returning())
            .get_result(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))
    }

    async fn delete(&self, post_id: Uuid) -> Result<(), ApiError> {
        use crate::infrastructure::database::schema::posts::dsl::*;

        let mut conn = self
            .pool
            .get()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        diesel::delete(posts.filter(id.eq(post_id)))
            .execute(&mut conn)
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl PostRepository for Arc<PostRepositoryImpl> {
    async fn find(&self, id: Uuid) -> Result<Post, ApiError> {
        self.as_ref().find(id).await
    }

    async fn find_all(&self) -> Result<Vec<Post>, ApiError> {
        self.as_ref().find_all().await
    }

    async fn create(&self, post: Post) -> Result<Post, ApiError> {
        self.as_ref().create(post).await
    }

    async fn update(&self, post_id: Uuid, post: Post) -> Result<Post, ApiError> {
        self.as_ref().update(post_id, post).await
    }

    async fn delete(&self, post_id: Uuid) -> Result<(), ApiError> {
        self.as_ref().delete(post_id).await
    }
}
