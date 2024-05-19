use std::future::Future; // Importing the Future trait from the standard library.
use std::pin::Pin; // Importing the Pin type from the standard library.
use std::task::{Context, Poll}; // Importing Context and Poll types from the standard library.

// Define a trait named Logging with a single method log.
trait Logging {
	fn log(&self);
}

// Define a struct LoggingFuture that wraps a future which also implements the Logging trait.
struct LoggingFuture<F: Future + Logging> {
	inner: F,
}

// Implement the Future trait for LoggingFuture.
impl<F: Future + Logging> Future for LoggingFuture<F> {
	type Output = F::Output; // The associated output type is the same as the inner future's output type.

	// Implement the poll method required by the Future trait.
	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		// Use unsafe to create a pinned mutable reference to the inner future.
		let inner = unsafe { self.map_unchecked_mut(|s| &mut s.inner) };
		inner.log(); // Call the log method before polling the inner future.
		inner.poll(cx) // Delegate the poll call to the inner future.
	}
}

// Implement the Logging trait for all types that implement Future.
// This provides a default log implementation for any future.
impl<G: Future> Logging for G {
	fn log(&self) {
		println!("Polling the future!"); // Print a message when log is called.
	}
}

// Define an asynchronous function that returns a String.
async fn my_async_function() -> String {
	"Result of async computation".to_string()
}

// The main function is the entry point of the program.
// The #[tokio::main] attribute sets up the Tokio runtime.
#[tokio::main]
async fn main() {
	// Create a LoggingFuture wrapping the my_async_function future.
	let logged_future = LoggingFuture {
		inner: my_async_function(),
	};
	// Await the result of the LoggingFuture, which will log and then execute the inner future.
	let result = logged_future.await;
	println!("{}", result); // Print the result of the asynchronous computation.
}
