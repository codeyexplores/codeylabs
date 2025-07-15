// rss fetcher
// use rss crate to parse articles,
// tokio-cron-scheduler for running scraping - 10-30mins
// source example feeds

// rss fetcher
use chrono::{Duration, Utc};
use rss::Channel;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};

/// Starts scheduled tasks (fetch + cleanup)
pub async fn start_cron_jobs(pool: Arc<SqlitePool>) {
    let mut scheduler = JobScheduler::new().await.unwrap();

    // Run every 30 minutes
    let fetch_pool = pool.clone();
    let fetch_job = Job::new_async("0 */30 * * * *", move |_, _| {
        let fetch_pool = fetch_pool.clone();
        Box::pin(async move {
            println!("⏰ Fetching latest articles...");
            if let Err(e) = fetch_and_store_articles(&fetch_pool).await {
                eprintln!("❌ Failed to fetch articles: {}", e);
            }
        })
    })
    .unwrap();

    // Clean up old articles daily at midnight
    let cleanup_pool = pool.clone();
    let cleanup_job = Job::new_async("0 0 0 * * *", move |_, _| {
        let cleanup_pool = cleanup_pool.clone();
        Box::pin(async move {
            println!("🧹 Cleaning up old articles...");
            let cutoff = (Utc::now() - Duration::days(30)).to_rfc3339();
            if let Err(e) = sqlx::query!("DELETE FROM articles WHERE published < ?", cutoff)
                .execute(&*cleanup_pool)
                .await
            {
                eprintln!("❌ Cleanup failed: {}", e);
            } else {
                println!("✅ Old articles removed.");
            }
        })
    })
    .unwrap();

    scheduler.add(fetch_job).await.unwrap();
    scheduler.add(cleanup_job).await.unwrap();
    scheduler.start().await.unwrap();
}

/// Fetch and store RSS articles
pub async fn fetch_and_store_articles(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let feed_urls = vec![
        "https://www.channelnewsasia.com/api/v1/rss-outbound-feed?_format=xml&category=10416", // sg
        "https://www.channelnewsasia.com/api/v1/rss-outbound-feed?_format=xml&category=6311", // world
        "https://www.channelnewsasia.com/api/v1/rss-outbound-feed?_format=xml&category=6936", // business
        "https://feeds.reuters.com/reuters/businessNews",
    ];

    for url in feed_urls {
        let content = match reqwest::get(url).await {
            Ok(res) => res.bytes().await?,
            Err(e) => {
                eprintln!("⚠️ Failed to fetch {}: {}", url, e);
                continue;
            }
        };

        let channel = match Channel::read_from(&content[..]) {
            Ok(chan) => chan,
            Err(e) => {
                eprintln!("⚠️ Failed to parse feed from {}: {}", url, e);
                continue;
            }
        };

        for item in channel.items() {
            let title = item.title().unwrap_or("No title");
            let link = item.link().unwrap_or("No link");
            let published = item.pub_date().unwrap_or("");
            let summary = item
                .description()
                .map(|s| s.to_string())
                .or_else(|| item.content().map(|s| s.to_string()));

            let _ = sqlx::query!(
                r#"
                INSERT OR IGNORE INTO articles (title, link, published, summary)
                VALUES (?, ?, ?, ?)
                "#,
                title,
                link,
                published,
                summary,
            )
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}