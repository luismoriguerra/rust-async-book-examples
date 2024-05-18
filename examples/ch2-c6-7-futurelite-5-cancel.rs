use async_io::Timer;
use futures_lite::Future;
use futures_lite::{future, pin};
use std::task::Poll;
use std::time::Duration;

async fn async_task(name: &str, delay: Duration) {
	Timer::after(delay).await;
	println!("Task {} completed!", name);
}

async fn cancelable_task() -> Result<(), &'static str> {
	let task = async_task("Cancelable Task", Duration::from_secs(5));
	let timeout = Timer::after(Duration::from_secs(2));

	pin!(task);
	pin!(timeout);

	future::poll_fn(
		|cx| match (
			task.as_mut().poll(cx), 
			timeout.as_mut().poll(cx)
		) {
			(Poll::Ready(_), _) => Poll::Ready(Ok(())),
			(_, Poll::Ready(_)) => Poll::Ready(Err("Task timed out")),
			_ => Poll::Pending,
		},
	)
	.await
}

fn main() {
	future::block_on(async {
		match cancelable_task().await {
			Ok(_) => println!("Task completed successfully"),
			Err(e) => println!("Error: {}", e),
		}
	});
}
