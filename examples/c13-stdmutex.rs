use std::{
	future::Future,
	pin::Pin,
	sync::{Arc, Mutex},
	task::{Context, Poll},
	time::Duration,
};

use tokio::task::JoinHandle;

#[derive(Debug)]
enum ActionType {
	Increment,
	Decrement,
}

struct SharedData {
	counter: i32,
}

impl SharedData {
	fn increment(&mut self) {
		self.counter += 1;
	}
	fn decrement(&mut self) {
		self.counter -= 1;
	}
}

struct CounterFuture {
	action_type: ActionType,
	unlock_shared_data_ref: Arc<Mutex<SharedData>>,
	internal_count_state: u32,
}

impl Future for CounterFuture {
	type Output = u32;
	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		std::thread::sleep(Duration::from_secs(1));
		let mut lock_shared_data_ref = match self.unlock_shared_data_ref.try_lock() {
			Ok(guard) => guard,
			Err(error) => {
				println!("error for {:?}: {}", self.action_type, error);
				cx.waker().wake_by_ref();
				return Poll::Pending;
			}
		};
		// program State is updated here
		let shared_data = &mut *lock_shared_data_ref;
		match self.action_type {
			ActionType::Increment => {
				shared_data.increment();
				println!("after increment: {}", shared_data.counter);
			}
			ActionType::Decrement => {
				shared_data.decrement();
				println!("after decrement: {}", shared_data.counter);
			}
		}
		// Releases the lock on the shared
		std::mem::drop(lock_shared_data_ref);
		// update internal state to keep track of the number of times the future has been polled
		self.internal_count_state += 1;
		if self.internal_count_state < 3 {
			cx.waker().wake_by_ref();
			return Poll::Pending;
		} else {
			return Poll::Ready(self.internal_count_state);
		}
	}
}

#[tokio::main]
async fn main() {
	// Arc is used to share the data between the two futures, threads, or tasks
	// Mutex is used to ensure that only one future can access the shared data at a time
	let shared_state = Arc::new(Mutex::new(SharedData { counter: 0 }));
	let counter_one = CounterFuture {
		action_type: ActionType::Increment,
		// clone the reference to the shared data
		unlock_shared_data_ref: shared_state.clone(),
		internal_count_state: 0,
	};
	let counter_two = CounterFuture {
		action_type: ActionType::Decrement,
		unlock_shared_data_ref: shared_state.clone(),
		internal_count_state: 0,
	};
	let handle_one = tokio::task::spawn(async move { counter_one.await });
	let handle_two = tokio::task::spawn(async move { counter_two.await });

	tokio::join!(handle_one, handle_two);
}

// error for Increment: try_lock failed because the operation would block
// after decrement: -1
// after decrement: -2
// error for Increment: try_lock failed because the operation would block
// after increment: -1
// after decrement: -2
// after increment: -1
// after increment: 0
