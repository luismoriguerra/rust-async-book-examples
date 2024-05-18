use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use std::{future::Future, panic::catch_unwind, thread};

use async_task::{Runnable, Task};
use flume::{Receiver, Sender};
use futures_lite::future;
use once_cell::sync::Lazy;

static HIGH_CHANNEL: Lazy<(Sender<Runnable>, Receiver<Runnable>)> =
	Lazy::new(|| flume::unbounded::<Runnable>());
static LOW_CHANNEL: Lazy<(Sender<Runnable>, Receiver<Runnable>)> =
	Lazy::new(|| flume::unbounded::<Runnable>());

macro_rules! spawn_task {
	($future:expr) => {
		spawn_task!($future, FutureType::Low)
	};
	($future:expr, $order:expr) => {
		spawn_task($future, $order)
	};
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

macro_rules! try_join {
    ($($future:expr),*) => {
        {
            let mut results = Vec::new();
            $(
                let result = catch_unwind(|| future::block_on($future));
                results.push(result);
            )*
            results
        }
    };
}

fn spawn_task<F, T>(future: F, order: FutureType) -> Task<T>
where
	F: Future<Output = T> + Send + 'static,
	T: Send + 'static,
{
	let schedule_high = |runnable| HIGH_QUEUE.send(runnable).unwrap();
	let schedule_low = |runnable| LOW_QUEUE.send(runnable).unwrap();

	let schedule = match order {
		FutureType::High => schedule_high,
		FutureType::Low => schedule_low,
	};
	let (runnable, task) = async_task::spawn(future, schedule);
	runnable.schedule();
	return task;
}

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

struct CounterFuture {
	count: u32,
}

static HIGH_QUEUE: Lazy<flume::Sender<Runnable>> = Lazy::new(|| {
	for _ in 0..2 {
		let high_receiver = HIGH_CHANNEL.1.clone();
		let low_receiver = LOW_CHANNEL.1.clone();
		thread::spawn(move || loop {
			match high_receiver.try_recv() {
				Ok(runnable) => {
					let _ = catch_unwind(|| runnable.run());
				}
				Err(_) => match low_receiver.try_recv() {
					Ok(runnable) => {
						let _ = catch_unwind(|| runnable.run());
					}
					Err(_) => {
						thread::sleep(Duration::from_millis(100));
					}
				},
			};
		});
	}
	HIGH_CHANNEL.0.clone()
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
	let one = CounterFuture { count: 0 };
	let two = CounterFuture { count: 0 };

	let t_one = spawn_task!(one, FutureType::High);
	let t_two = spawn_task!(two);
	let t_three = spawn_task!(async_fn());
	let t_four = spawn_task!(
		async {
			async_fn().await;
			async_fn().await;
		},
		FutureType::High
	);

	// future::block_on(t_one);
	// future::block_on(t_two);
	// future::block_on(t_three);
	// future::block_on(t_four);
	let outcome: Vec<u32> = join!(t_one, t_two);
	let outcome_two: Vec<()> = join!(t_four, t_three);
}
