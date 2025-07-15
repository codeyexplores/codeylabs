// db init + queries
// connects to db

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub async fn init_db() -> SqlitePool {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = SqlitePoolOptions::new()
        .max_connections(20) // modify max connections
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    // Create articles table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS articles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            link TEXT NOT NULL UNIQUE,
            published TEXT,
            summary TEXT
        )
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to create articles table");

    sqlx::query!(
        r#"
        DELETE FROM articles
        WHERE published < date('now', '-30 days')
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to delete old articles");

    pool
}