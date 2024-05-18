use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

// Function that might fail during initialization
fn initialize_config() -> Result<HashMap<String, String>, &'static str> {
	let mut m = HashMap::new();
	m.insert("key".to_string(), "value".to_string());
	if m.is_empty() {
		Err("Initialization failed")
	} else {
		Ok(m)
	}
}

// Create a lazily initialized static variable with a Mutex
static CONFIG: Lazy<Mutex<Result<HashMap<String, String>, &'static str>>> =
	Lazy::new(|| Mutex::new(initialize_config()));

fn main() {
	match &*CONFIG.lock().unwrap() {
		Ok(config) => println!("CONFIG: {:?}", config),
		Err(e) => println!("Failed to initialize CONFIG: {}", e),
	}
}
