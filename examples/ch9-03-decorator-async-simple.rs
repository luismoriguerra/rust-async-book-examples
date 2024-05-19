use std::future::Future;

// Function to log and poll a future without wrapping it in a struct.
async fn log_and_run<F: Future>(fut: F) -> F::Output {
	println!("Polling the future!");
	fut.await
}

async fn my_async_function() -> String {
	"Result of async computation".to_string()
}

#[tokio::main]
async fn main() {
	let result = log_and_run(my_async_function()).await;
	println!("{}", result);
}
