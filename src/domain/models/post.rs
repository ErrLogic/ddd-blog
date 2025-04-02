use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::infrastructure::database::schema::posts;

#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub author_id: Uuid,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub author_id: Uuid,
    pub published: bool,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = posts)]
pub struct UpdatePostData {
    pub title: Option<String>,
    pub content: Option<String>,
    pub published: Option<bool>,
    #[diesel(column_name = "updated_at")]
    pub updated_at: Option<NaiveDateTime>,
}

// Your existing CreatePost and UpdatePost structs remain the same
#[derive(Debug, Validate, Deserialize)]
pub struct CreatePost {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
    pub author_id: Uuid,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdatePost {
    #[validate(length(min = 1, max = 100))]
    pub title: Option<String>,
    #[validate(length(min = 1))]
    pub content: Option<String>,
    pub published: Option<bool>,
}
