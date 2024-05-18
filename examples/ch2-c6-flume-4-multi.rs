use flume::unbounded;
use std::thread;

fn main() {
	let (tx, rx) = unbounded();

	// Spawn multiple producers
	let producers: Vec<_> = (0..3)
		.map(|row| {
			let tx = tx.clone();
			thread::spawn(move || {
				for value in 0..5 {
					tx.send((row, value)).unwrap();
					println!("Producer {}: Produced {}", row, value);
				}
			})
		})
		.collect();

	// Spawn multiple consumers
	let consumers: Vec<_> = (0..2)
		.map(|i| {
			let rx = rx.clone();
			thread::spawn(move || {
				while let Ok((producer, value)) = rx.recv() {
					println!(
						"Consumer {}: Consumed {} from producer {}",
						i, value, producer
					);
				}
			})
		})
		.collect();

	for producer in producers {
		producer.join().unwrap();
	}

	drop(tx); // Close the channel

	for consumer in consumers {
		consumer.join().unwrap();
	}
}
