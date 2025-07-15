// articles struct and DB logic
// creates the rust Article struct

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Article {
    pub id: i64,
    pub title: String,
    pub link: String,
    pub published: Option<String>,
    pub summary: Option<String>,
}

/*
FromRow: For use with sqlx::query_as!

Serialize: Required to return Json(article)

Deserialize: Optional but useful if you accept article creation in the future
*/