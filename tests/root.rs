use std::env;

#[tokio::test]
async fn get_root() -> Result<(), reqwest::Error> {
    let default_url = String::from("http://localhost:3000");
    let url = env::var("url").unwrap_or(default_url);
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;

    assert_eq!(body, "Hello, World!");

    Ok(())
}