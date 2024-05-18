use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use std::{future::Future, panic::catch_unwind, thread};

use async_task::{Runnable, Task};
use futures_lite::future;
use once_cell::sync::Lazy;

static QUEUE: Lazy<flume::Sender<Runnable>> = Lazy::new(|| {
	let (tx, rx) = flume::unbounded::<Runnable>();

	for _ in 0..3 {
		let receiver = rx.clone();
		thread::spawn(move || {
			while let Ok(runnable) = receiver.recv() {
				let _ = catch_unwind(|| runnable.run());
			}
		});
	}

	tx
});

fn spawn_task<F, T>(future: F) -> Task<T>
where
	F: Future<Output = T> + Send + 'static + FutureOrderLabel,
	T: Send + 'static,
{
	// let schedule = |runnable| QUEUE.send(runnable).unwrap();
	// let (runnable, task) = async_task::spawn(future, schedule);

	// runnable.schedule();
	// println!("Here is the queue count: {:?}", QUEUE.len());
	// return task;

	let schedule_high = |runnable| HIGH_QUEUE.send(runnable).unwrap();
	let schedule_low = |runnable| LOW_QUEUE.send(runnable).unwrap();

	let schedule = match future.get_order() {
		FutureType::High => schedule_high,
		FutureType::Low => schedule_low,
	};
	let (runnable, task) = async_task::spawn(future, schedule);
	runnable.schedule();
	return task;
}

// struct CounterFuture {
// 	count: u32,
// }
impl Future for CounterFuture {
	type Output = u32;

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		self.count += 1;
		println!("polling with result: {}", self.count);
		std::thread::sleep(Duration::from_secs(1));
		if self.count < 3 {
			cx.waker().wake_by_ref();
			Poll::Pending
		} else {
			Poll::Ready(self.count)
		}
	}
}

async fn async_fn() {
	std::thread::sleep(Duration::from_secs(1));
	println!("async fn");
}

use std::time::Instant;

struct AsyncSleep {
	start_time: Instant,
	duration: Duration,
}
impl AsyncSleep {
	fn new(duration: Duration) -> Self {
		Self {
			start_time: Instant::now(),
			duration,
		}
	}
}

impl Future for AsyncSleep {
	type Output = bool;

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		let elapsed_time = self.start_time.elapsed();
		if elapsed_time >= self.duration {
			Poll::Ready(true)
		} else {
			cx.waker().wake_by_ref();
			Poll::Pending
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum FutureType {
	High,
	Low,
}
trait FutureOrderLabel: Future {
	fn get_order(&self) -> FutureType;
}

struct CounterFuture {
	count: u32,
	order: FutureType,
}

impl FutureOrderLabel for CounterFuture {
	fn get_order(&self) -> FutureType {
		self.order
	}
}

static HIGH_QUEUE: Lazy<flume::Sender<Runnable>> = Lazy::new(|| {
	let (tx, rx) = flume::unbounded::<Runnable>();
	for _ in 0..2 {
		let receiver = rx.clone();
		thread::spawn(move || {
			while let Ok(runnable) = receiver.recv() {
				let _ = catch_unwind(|| runnable.run());
			}
		});
	}
	tx
});
static LOW_QUEUE: Lazy<flume::Sender<Runnable>> = Lazy::new(|| {
	let (tx, rx) = flume::unbounded::<Runnable>();
	for _ in 0..1 {
		let receiver = rx.clone();
		thread::spawn(move || {
			while let Ok(runnable) = receiver.recv() {
				let _ = catch_unwind(|| runnable.run());
			}
		});
	}
	tx
});

fn main() {
	let one = CounterFuture {
		count: 0,
		order: FutureType::High,
	};
	let two = CounterFuture {
		count: 0,
		order: FutureType::Low,
	};
	let t_one = spawn_task(one);
	let t_two = spawn_task(two);
	// let t_three = spawn_task(async {
	// 	async_fn().await;
	// 	async_fn().await;
	// 	async_fn().await;
	// 	async_fn().await;
	// });
	std::thread::sleep(Duration::from_secs(5));
	println!("before the block");
	future::block_on(t_one);
	future::block_on(t_two);
	// future::block_on(t_three);
}
