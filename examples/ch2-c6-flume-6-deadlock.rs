use flume::unbounded;
use std::thread;

fn main() {
	let (tx, rx) = unbounded();

	let tx2 = tx.clone();
	let producer = thread::spawn(move || {
		for i in 0..5 {
			if tx2.send(i).is_err() {
				println!("Consumer has dropped, stopping production.");
				return;
			}
			println!("Produced: {}", i);
		}
	});

	let consumer = thread::spawn(move || {
		for value in rx.iter() {
			println!("Consumed: {}", value);
		}
		println!("Producer has stopped.");
	});

	producer.join().unwrap();
	drop(tx); // Close the channel
	consumer.join().unwrap();
}
