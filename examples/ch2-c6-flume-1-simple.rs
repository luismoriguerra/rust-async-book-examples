use flume::unbounded;
use std::thread;

fn main() {
	// Create an unbounded channel
	let (tx, rx) = unbounded();

	// Spawn a producer thread
	let producer = thread::spawn(move || {
		for i in 0..100 {
			tx.send(i).unwrap();
			println!("Produced: {}", i);
		}
	});

	// Spawn a consumer thread
	let consumer = thread::spawn(move || {
		while let Ok(value) = rx.recv() {
			println!("Consumed: {}", value);
		}
	});

	producer.join().unwrap();
	consumer.join().unwrap();
}
