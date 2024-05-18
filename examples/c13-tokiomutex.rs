use std::{sync::Arc, time::Duration};

use tokio::task::JoinHandle;

#[derive(Debug)]
enum ActionType {
	Increment,
	Decrement,
}

struct SharedState {
	counter: i32,
}

impl SharedState {
	fn increment(&mut self) {
		self.counter += 1;
	}
	fn decrement(&mut self) {
		self.counter -= 1;
	}
}

async fn count(
	internal_state_count: u32,
	unlock_shared_state: Arc<tokio::sync::Mutex<SharedState>>,
	action_type: ActionType,
) -> u32 {
	for _ in 0..internal_state_count {
		let mut lock_shared_state = unlock_shared_state.lock().await;

		match action_type {
			ActionType::Increment => {
				lock_shared_state.increment();
				println!("after increment: {}", lock_shared_state.counter);
			}
			ActionType::Decrement => {
				lock_shared_state.decrement();
				println!("after decrement: {}", lock_shared_state.counter);
			}
		}
		std::mem::drop(lock_shared_state);
		std::thread::sleep(Duration::from_secs(1));
	}
	return internal_state_count;
}

#[tokio::main]
async fn main() {
	let shared_state = Arc::new(tokio::sync::Mutex::new(SharedState { counter: 0 }));
	let shared_state_ref_two = shared_state.clone();

	let handle_one = tokio::task::spawn(async move {
		count(3, shared_state, ActionType::Increment).await
	});
	let handle_two = tokio::task::spawn(async move {
		count(3, shared_state_ref_two, ActionType::Decrement).await
	});
	tokio::join!(handle_one, handle_two);
}
