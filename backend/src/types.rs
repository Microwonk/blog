use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// TODO: add archived bool to posts, so they can easily be hidden.archived
#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: i32,
    pub author: i32,
    pub title: String,
    pub description: String,
    pub slug: String,
    pub markdown_content: String,
    pub released: bool,
    pub release_date: Option<sqlx::types::chrono::NaiveDateTime>,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
    pub updated_at: Option<sqlx::types::chrono::NaiveDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub description: String,
    pub markdown_content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct ProcessedPost {
    pub id: i32,
    pub author: i32,
    pub author_name: String,
    pub title: String,
    pub description: String,
    pub slug: String,
    pub markdown_content: String,
    pub released: bool,
    pub release_date: Option<sqlx::types::chrono::NaiveDateTime>,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
    pub updated_at: Option<sqlx::types::chrono::NaiveDateTime>,
}

// TODO: profile picture

#[derive(Clone, Debug, Serialize, Deserialize, FromRow, Default)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub admin: bool,
    pub passwordhash: String,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
}

impl User {
    /// clones self and makes a UserProfile instance
    pub fn profile(&self) -> UserProfile {
        let cloned = self.clone();
        UserProfile {
            id: cloned.id,
            name: cloned.name,
            email: cloned.email,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    // pub admin: bool,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct UserProfile {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserUpdate {
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Media {
    pub id: i32,
    pub post_id: i32,
    pub name: String,
    pub data: Vec<u8>,
    pub media_type: String,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct MediaNoData {
    pub id: i32,
    pub post_id: i32,
    pub name: String,
    pub media_type: String,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: i32,
    pub author: Option<i32>,
    pub post: i32,
    pub content: String,
    pub replying_to: Option<i32>,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow, Default)]
pub struct ProcessedComment {
    pub id: i32,
    pub author_name: Option<String>,
    pub author_id: Option<i32>,
    pub content: String,
    pub replying_to: Option<i32>,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewComment {
    pub content: String,
    pub replying_to: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IsAdminResponse {
    pub admin: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow, Default)]
pub struct LogEntry {
    pub id: i32,
    pub message: String,
    pub context: String,
    pub log_time: sqlx::types::chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct NewLogEntry {
    pub message: String,
    pub context: String,
}

impl NewLogEntry {
    pub fn new(message: String, context: String) -> Self {
        Self { message, context }
    }
}

pub struct RssEntry {
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: String,
    pub author: String,
    pub guid: String,
}

impl From<ProcessedPost> for RssEntry {
    fn from(post: ProcessedPost) -> Self {
        let full_url = format!("https://blog.nicolas-frey.com/posts/{}", post.slug);
        let perm_url = format!("https://blog.nicolas-frey.com/posts/{}", post.id);
        Self {
            title: post.title,
            link: full_url,
            description: post.description,
            pub_date: Utc
                .from_utc_datetime(
                    &post
                        .release_date
                        .unwrap_or(post.updated_at.unwrap_or(post.created_at)),
                )
                .to_rfc2822(),
            author: post.author_name,
            guid: perm_url,
        }
    }
}

impl RssEntry {
    // Converts an RSSEntry to a String containing the rss item tags
    pub fn to_item(&self) -> String {
        format!(
            r#"
        <item>
            <title><![CDATA[{}]]></title>
            <description><![CDATA[{}]]></description>
            <pubDate>{}</pubDate>
            <link>{}</link>
            <guid isPermaLink="true">{}</guid>
            <author>{}</author>
        </item>
      "#,
            self.title, self.description, self.pub_date, self.link, self.guid, self.author
        )
    }
}
