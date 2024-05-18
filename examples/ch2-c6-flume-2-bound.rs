use flume::bounded;
use std::thread;
use std::time::Duration;

fn main() {
	// Create a bounded channel with capacity 2
	let (tx, rx) = bounded(2);

	// Spawn a producer thread
	let producer = thread::spawn(move || {
		for i in 0..5 {
			tx.send(i).unwrap();
			println!("Produced: {}", i);
			thread::sleep(Duration::from_millis(100));
		}
	});

	// Spawn a consumer thread
	let consumer = thread::spawn(move || {
		while let Ok(value) = rx.recv() {
			println!("Consumed: {}", value);
			thread::sleep(Duration::from_millis(200));
		}
	});

	producer.join().unwrap();
	consumer.join().unwrap();
}
