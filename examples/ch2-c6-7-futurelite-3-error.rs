use futures::join;
use futures_lite::future;
use std::time::Duration;

async fn async_task(name: &str, delay: Duration) -> Result<(), &'static str> {
	if delay.as_secs() > 1 {
		return Err("Delay too long");
	}
	std::thread::sleep(delay);
	println!("Task {} completed!", name);
	Ok(())
}

fn main() {
	let task1 = async_task("Task 1", Duration::from_secs(1));
	let task2 = async_task("Task 2", Duration::from_secs(2));

	future::block_on(async {
		match join!(task1, task2) {
			(Ok(_), Ok(_)) => println!("All tasks completed successfully"),
			(Err(e), _) | (_, Err(e)) => println!("Error: {}", e),
		}
	});
}
