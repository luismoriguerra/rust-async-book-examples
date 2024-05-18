use axum::{routing::get, Router};
use once_cell::sync::OnceCell;
use reqwest;
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tokio::sync::OnceCell as AsyncOnceCell;

// Configuration struct
#[derive(Debug, Deserialize, Clone)]
struct Config {
	database_url: String,
	port: u16,
}

// Global configuration instance
static CONFIG: OnceCell<Arc<RwLock<Config>>> = OnceCell::new();

#[tokio::main]
async fn main() {
	// Asynchronously fetch the configuration
	let config: Config = reqwest::get("https://example.com/config")
		.await
		.expect("Failed to fetch config")
		.json()
		.await
		.expect("Failed to parse config");

	CONFIG
		.set(Arc::new(RwLock::new(config)))
		.expect("Failed to set configuration");

	let app = Router::new().route("/", get(handler));

	let port = CONFIG
		.get()
		.expect("Configuration not set")
		.read()
		.unwrap()
		.port;
	let addr = SocketAddr::from(([127, 0, 0, 1], port));

	println!("Listening on {}", addr);

	tokio::spawn(reload_config());

	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}

async fn handler() -> &'static str {
	let config = CONFIG.get().expect("Configuration not set").read().unwrap();
	format!(
		"Database URL: {}, Port: {}",
		config.database_url, config.port
	)
}

// Function to reload the configuration
async fn reload_config() {
	loop {
		tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
		let new_config: Config = reqwest::get("https://example.com/config")
			.await
			.expect("Failed to fetch config")
			.json()
			.await
			.expect("Failed to parse config");

		let mut config = CONFIG
			.get()
			.expect("Configuration not set")
			.write()
			.unwrap();
		*config = new_config;
	}
}
