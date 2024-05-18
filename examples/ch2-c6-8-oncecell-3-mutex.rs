use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

// Create a lazily initialized static variable with a Mutex
static CONFIG: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
	let mut m = HashMap::new();
	m.insert("key".to_string(), "value".to_string());
	Mutex::new(m)
});

fn main() {
	{
		let mut config = CONFIG.lock().unwrap();
		config.insert("new_key".to_string(), "new_value".to_string());
	}

	{
		let config = CONFIG.lock().unwrap();
		println!("CONFIG: {:?}", *config);
	}
}
