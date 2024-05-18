use flume::{bounded, select};
use std::thread;
use std::time::Duration;

fn main() {
	let (tx1, rx1) = bounded(1);
	let (tx2, rx2) = bounded(1);

	thread::spawn(move || {
		tx1.send("Message from channel 1").unwrap();
	});

	thread::spawn(move || {
		tx2.send("Message from channel 2").unwrap();
	});

	flume::Selector::new()
		.recv(&rx1, |msg| {
			println!("Received from channel 1: {:?}", msg);
		})
		.recv(&rx2, |msg| {
			println!("Received from channel 2: {:?}", msg);
		})
		.wait();
}
