use std::env;

use maud::html;

#[tokio::test]
async fn punch_in_punch_out() -> Result<(), reqwest::Error> {
    let default_url = String::from("http://localhost:3000");
    let url = env::var("url").unwrap_or(default_url);
    let client = reqwest::Client::new();

    let body = client.get(format!("{url}/"))
        .send()
        .await?
        .text()
        .await?;

    if !body.contains("punched out") {
        panic!("expected body to contain text '{}': \n{}", "punched out", body);
    }

    let body = client.post(format!("{url}/punch"))
        .send()
        .await?
        .text()
        .await?;

    let expected_markup = html! { "punched in" };
    assert_eq!(body, expected_markup.into_string());

    let body = client.get(format!("{url}/"))
        .send()
        .await?
        .text()
        .await?;

    if !body.contains("punched in") {
        panic!("expected body to contain text '{}': \n{}", "punched in", body);
    }

    let body = client.post(format!("{url}/punch"))
        .send()
        .await?
        .text()
        .await?;
    let expected_markup = html! { "punched out" };

    assert_eq!(body, expected_markup.into_string());

    Ok(())
}