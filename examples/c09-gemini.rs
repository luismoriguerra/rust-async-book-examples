use std::time::Duration;
use tokio::{join, task};

async fn my_task(task_name: &str, delay_ms: u64) {
	tokio::time::sleep(Duration::from_millis(delay_ms)).await;
	println!("{} completed!", task_name);
}

#[tokio::main]
async fn main() {
	// Using tokio::join! for concurrency
	tokio::join!(
		my_task("Task 1", 200),
		my_task("Task 2", 150),
		my_task("Task 3", 300)
	);
	println!("All tasks finished (tokio::join!)");

	// Using tokio::task::spawn for independent tasks
	let task_handle1 = tokio::task::spawn(my_task("Task A", 100));
	let task_handle2 = tokio::task::spawn(my_task("Task B", 50));

	// You can await handles independently, or not at all
	task_handle1.await.unwrap();
	println!("Task A finished (tokio::task::spawn)");
}

// Task 2 completed!
// Task 1 completed!
// Task 3 completed!
// All tasks finished (tokio::join!)
// Task B completed!
// Task A completed!
// Task A finished (tokio::task::spawn)
