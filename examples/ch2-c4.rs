use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use std::{future::Future, panic::catch_unwind, thread};

use async_task::{Runnable, Task};
use flume::{Receiver, Sender};
use futures_lite::future;
use once_cell::sync::Lazy;
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

struct Runtime {
	high_num: usize,
	low_num: usize,
}

impl Runtime {
	pub fn new() -> Self {
		let num_cores = std::thread::available_parallelism().unwrap().get();
		Self {
			high_num: num_cores - 2,
			low_num: 1,
		}
	}
	pub fn with_high_num(mut self, num: usize) -> Self {
		self.high_num = num;
		self
	}
	pub fn with_low_num(mut self, num: usize) -> Self {
		self.low_num = num;
		self
	}
	pub fn run(&self) {
		std::env::set_var("HIGH_NUM", self.high_num.to_string());
		std::env::set_var("LOW_NUM", self.low_num.to_string());

		let high = spawn_task!(async {}, FutureType::High);
		let low = spawn_task!(async {}, FutureType::Low);
		join!(high, low);
	}
}

#[derive(Debug, Clone, Copy)]
enum FutureType {
	High,
	Low,
}
static HIGH_CHANNEL: Lazy<(Sender<Runnable>, Receiver<Runnable>)> =
	Lazy::new(|| flume::unbounded::<Runnable>());

static LOW_CHANNEL: Lazy<(Sender<Runnable>, Receiver<Runnable>)> =
	Lazy::new(|| flume::unbounded::<Runnable>());

static HIGH_QUEUE: Lazy<flume::Sender<Runnable>> = Lazy::new(|| {
	let high_num = std::env::var("HIGH_NUM").unwrap().parse::<usize>().unwrap();

	for _ in 0..high_num {
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

fn main() {}
