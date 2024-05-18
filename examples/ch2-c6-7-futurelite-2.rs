use futures_lite::future;
use std::time::Duration;

async fn async_task(name: &str, delay: Duration) {
	std::thread::sleep(delay);
	println!("Task {} completed!", name);
}

macro_rules! join {
    ($($future:expr),*) => {
        {
            let mut results = Vec::new();
            $(
                results.push(future::block_on($future));
            )*
            results
        }
    };
}

fn main() {
	let task1 = async_task("Task 1", Duration::from_secs(1));
	let task2 = async_task("Task 2", Duration::from_secs(2));

	join!(task1, task2);
}
