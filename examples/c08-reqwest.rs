use reqwest::Error;
use serde::Deserialize;
use serde_json;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Deserialize, Debug)]
struct Response {
	url: String,
	args: serde_json::Value,
}

async fn fetch_data(senconds: u64) -> Result<Response, Error> {
	let request_url = format!("https://httpbin.org/delay/{}", senconds);
	let response = reqwest::get(&request_url).await?;

	let delay_response = response.json().await?;

	Ok(delay_response)
}

async fn calculate_last_login() {
	sleep(Duration::from_secs(1)).await;
	println!("Logged in 2 days ago");
}

#[tokio::main]
async fn main() -> Result<(), Error> {
	let data = fetch_data(5);
	let time_since = calculate_last_login();
	let (posts, _) = tokio::join!(data, time_since);
	println!("Fetched {:?}", posts);
	Ok(())
}
