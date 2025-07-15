// openai summarizer
// maybe explore rust crate
// OPENAI_API_KEY
// prompt logic might be - summarize this article in 4 sentences [ title, description ]


use async_openai::{
    Client,
    types::{
        ChatCompletionRequestMessage, 
        CreateChatCompletionRequestArgs, 
        ChatCompletionRequestSystemMessage, 
        ChatCompletionRequestUserMessage,
        ChatCompletionRequestSystemMessageContent,
        ChatCompletionRequestUserMessageContent
    },
};
use anyhow::Result;
use sqlx::SqlitePool;
use crate::models::Article;

pub async fn generate_summary(text: &str) -> Result<String> {
    // Create OpenAI client (auto-reads OPENAI_API_KEY env var)
    let client = Client::new();

    // Build chat messages using public constructors
    let messages = vec![
        ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
            content: ChatCompletionRequestSystemMessageContent::Text("You are a concise assistant that summarizes news articles in 2-3 sentences.".to_string()),
            name: None,
        }),
        ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
            content: ChatCompletionRequestUserMessageContent::Text(format!("Summarize this article:\n\n{}", text)),
            name: None,
        }),
    ];

    // Build chat completion request
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages(messages)
        .max_tokens(150u16)
        .temperature(0.7)
        .build()?;

    // Call OpenAI API
    let response = client.chat().create(request).await?;

    // Extract generated summary text
    let summary = response
        .choices
        .first()
        .and_then(|c| c.message.content.clone())
        .unwrap_or_else(|| "Summary not available.".to_string());

    Ok(summary)
}

pub async fn update_missing_summaries(db: &SqlitePool) -> Result<()> {
    let articles: Vec<Article> = sqlx::query_as!(
        Article,
        r#"
        SELECT id, title, link, published, summary
        FROM articles
        WHERE summary IS NULL OR summary = ''
        "#
    )
    .fetch_all(db)
    .await?;

    for article in articles {
        let text = format!("{} {}", article.title, article.link);
        match generate_summary(&text).await {
            Ok(summary) => {
                sqlx::query!(
                    r#"
                    UPDATE articles SET summary = ? WHERE id = ?
                    "#,
                    summary,
                    article.id
                )
                .execute(db)
                .await?;
                println!("✅ Summarized article: {}", article.title);
            }
            Err(e) => eprintln!("❌ Error summarizing '{}': {}", article.title, e),
        }
    }

    Ok(())
}

// use std::env;
// use sqlx::SqlitePool;
// use crate::models::Article;

// use openai::{
//     chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
//     auth::Credentials,
// };

// /// Generates a summary using OpenAI Chat API for the given text.
// pub async fn generate_summary(text: &str) -> Result<String, Box<dyn std::error::Error>> {
//     // Load credentials from OPENAI_KEY in .env
//     let credentials = Credentials::from_env();

//     let messages = vec![
//         ChatCompletionMessage {
//             role: ChatCompletionMessageRole::System,
//             content: Some("You are a concise assistant that summarizes news articles in 3-4 sentences.".to_string()),
//             name: None,
//             function_call: None,
//             tool_call_id: None,
//             tool_calls: None,
//         },
//         ChatCompletionMessage {
//             role: ChatCompletionMessageRole::User,
//             content: Some(format!("Summarize this news article:\n\n{}", text)),
//             name: None,
//             function_call: None,
//             tool_call_id: None,
//             tool_calls: None,
//         },
//     ];

//     let chat_completion = ChatCompletion::builder("gpt-4o", messages)
//         .credentials(credentials)
//         .create()
//         .await?;

//     let returned_message = chat_completion
//         .choices
//         .first()
//         .and_then(|choice| choice.message.content.clone())
//         .unwrap_or_else(|| "Summary not available.".to_string());

//     Ok(returned_message)
// }

// /// Updates all articles in the database that are missing summaries.
// pub async fn update_missing_summaries(db: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
//     let articles: Vec<Article> = sqlx::query_as!(
//         Article,
//         r#"
//         SELECT id, title, link, published, summary
//         FROM articles
//         WHERE summary IS NULL OR summary = ''
//         "#
//     )
//     .fetch_all(db)
//     .await?;

//     for article in articles {
//         let text = format!("{} {}", article.title, article.link);
//         match generate_summary(&text).await {
//             Ok(summary) => {
//                 sqlx::query!(
//                     r#"
//                     UPDATE articles SET summary = ? WHERE id = ?
//                     "#,
//                     summary,
//                     article.id
//                 )
//                 .execute(db)
//                 .await?;
//                 println!("✅ Summarized article: {}", article.title);
//             }
//             Err(e) => {
//                 eprintln!("❌ Error summarizing '{}': {}", article.title, e);
//             }
//         }
//     }

//     Ok(())
// }

