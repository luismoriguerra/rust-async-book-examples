use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::thread;

// Create a lazily initialized static variable
static CONFIG: Lazy<HashMap<String, String>> = Lazy::new(|| {
	let mut m = HashMap::new();
	m.insert("key1".to_string(), "value1".to_string());
	m.insert("key2".to_string(), "value2".to_string());
	m
});

fn main() {
	let handles: Vec<_> = (0..10)
		.map(|i| {
			thread::spawn(move || {
				println!("Thread {}: {:?}", i, CONFIG.get(&"key1".to_string()));
			})
		})
		.collect();

	for handle in handles {
		handle.join().unwrap();
	}
}
