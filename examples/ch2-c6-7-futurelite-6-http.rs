use async_std::task;
use futures_lite::future;
use std::sync::Arc;
use surf;

async fn fetch_url(url: String) -> Result<(), surf::Error> {
	let res = surf::get(&url).await?;
	println!("Fetched {}: {}", url, res.status());
	Ok(())
}

async fn fetch_multiple_urls(urls: Arc<Vec<String>>, batch_size: usize) {
	let mut tasks = Vec::new();
	for chunk in urls.chunks(batch_size) {
		for url in chunk.iter() {
			let url = url.clone();
			tasks.push(task::spawn(fetch_url(url)));
		}

		for task in tasks.drain(..) {
			if let Err(e) = task.await {
				eprintln!("Error fetching URL: {}", e);
			}
		}
	}
}

fn main() {
	let urls = vec![
		"https://www.rust-lang.org".to_string(),
		"https://www.github.com".to_string(),
		"https://www.google.com".to_string(),
		// Add more URLs as needed
	];

	let urls = Arc::new(urls);
	future::block_on(fetch_multiple_urls(urls, 2));
}
