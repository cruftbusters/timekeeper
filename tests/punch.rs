use std::env;

use maud::html;
use reqwest::StatusCode;

#[tokio::test]
async fn punch_in_punch_out() -> Result<(), reqwest::Error> {
    let default_url = String::from("http://localhost:3000");
    let url = env::var("url").unwrap_or(default_url);
    let client = reqwest::Client::new();

    let response = client.get(format!("{url}/")).send().await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.text().await?;
    if !body.contains("punched out") {
        panic!("expected body to contain text '{}': \n{}", "punched out", body);
    }

    let response = client.post(format!("{url}/punch")).send().await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.text().await?;
    let expected_markup = html! { "punched in" };
    assert_eq!(body, expected_markup.into_string());

    let response = client.get(format!("{url}/")).send().await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.text().await?;
    if !body.contains("punched in") {
        panic!("expected body to contain text '{}': \n{}", "punched in", body);
    }

    let response = client.post(format!("{url}/punch")).send().await?;
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.text().await?;
    let expected_markup = html! { "punched out" };
    assert_eq!(body, expected_markup.into_string());

    Ok(())
}