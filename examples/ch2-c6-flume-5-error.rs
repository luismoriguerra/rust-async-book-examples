use flume::{bounded, TryRecvError, TrySendError};
use std::thread;
use std::time::Duration;

fn main() {
	let (tx, rx) = bounded(2);

	let producer = thread::spawn(move || {
		for value in 0..5 {
			match tx.try_send(value) {
				Ok(_) => println!("Produced: {}", value),
				Err(TrySendError::Full(_)) => {
					println!("Channel is full, couldn't send {}", value)
				}
				Err(_) => (),
			}
			thread::sleep(Duration::from_millis(100));
		}
	});

	let consumer = thread::spawn(move || loop {
		match rx.try_recv() {
			Ok(value) => println!("Consumed: {}", value),
			Err(TryRecvError::Empty) => println!("Channel is empty, waiting..."),
			Err(TryRecvError::Disconnected) => break,
			Err(_) => (),
		}
		thread::sleep(Duration::from_millis(200));
	});

	producer.join().unwrap();
	consumer.join().unwrap();
}
