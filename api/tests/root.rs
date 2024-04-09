use std::env;
use maud::{DOCTYPE, html};

#[tokio::test]
async fn get_heartbeat() -> Result<(), reqwest::Error> {
    let default_url = String::from("http://localhost:3000");
    let url = env::var("url").unwrap_or(default_url);
    let request_url = format!("{url}/heartbeat");
    let body = reqwest::get(request_url)
        .await?
        .text()
        .await?;

    let expected_markup = html! {
        (DOCTYPE)
        html { "Hello, World!" }
    };

    assert_eq!(body, expected_markup.into_string());

    Ok(())
}