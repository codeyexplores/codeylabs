// api routes
// use REST API with axum
// GET /articles -> list
// GET /articles/:id -> single article
// return data in JSON for flutter

use crate::models;
use axum::{Json, extract::State, response::IntoResponse};
use rss::Channel;

use axum::http::StatusCode;
// use sqlx::{ sqlite::SqlitePoolOptions, SqlitePool };
use models::Article;
use sqlx::SqlitePool;

pub async fn fetch_rss_headlines(State(db): State<SqlitePool>) -> impl IntoResponse {
    // let feed_url = "http://seekingalpha.com/feed.xml";
    // let feed_url = "https://www.channelnewsasia.com/api/v1/rss-outbound-feed?_format=xml&category=10416"; // singapore
    let feed_url = "https://www.channelnewsasia.com/api/v1/rss-outbound-feed?_format=xml&category=679471"; // singapore


    match reqwest::get(feed_url).await {
        Ok(resp) => match resp.bytes().await {
            Ok(bytes) => match Channel::read_from(&bytes[..]) {
                Ok(channel) => {
                    let mut headlines = Vec::new();

                    for item in channel.items().iter() {
                        let title = item.title().unwrap_or_default();
                        let link = item.link().unwrap_or_default();
                        let published = item.pub_date().map(|s| s.to_string());
                        let summary = item
                            .description()
                            .map(|s| s.to_string())
                            .or_else(|| item.content().map(|s| s.to_string())); // Extract description if available

                        // Insert to DB with summary (can be None)
                        let _ = sqlx::query!(
                            r#"
                            INSERT OR IGNORE INTO articles (title, link, published, summary)
                            VALUES (?, ?, ?, ?)
                            "#,
                            title,
                            link,
                            published,
                            summary
                        )
                        .execute(&db)
                        .await;

                        headlines.push(title.to_string());
                    }

                    Json(headlines).into_response()
                }
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to parse RSS feed.",
                )
                    .into_response(),
            },
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read RSS feed body.",
            )
                .into_response(),
        },
        Err(_) => (StatusCode::BAD_GATEWAY, "Failed to fetch RSS feed.").into_response(),
    }
}

// Get stored articles
pub async fn get_articles(State(db): State<SqlitePool>) -> impl IntoResponse {
    let result = sqlx::query_as::<_, Article>(
        r#"
        SELECT id, title, link, published, summary
        FROM articles
        ORDER BY id DESC
        LIMIT 100
        "#,
    )
    .fetch_all(&db)
    .await;

    match result {
        Ok(articles) => Json(articles).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to fetch articles from DB.",
        )
            .into_response(),
    }
}
