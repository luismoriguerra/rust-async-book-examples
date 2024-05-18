use once_cell::sync::Lazy;
use std::collections::HashMap;

// Create a lazily initialized static variable
static CONFIG: Lazy<HashMap<String, String>> = Lazy::new(|| {
	let mut m = HashMap::new();
	m.insert("key".to_string(), "value".to_string());
	m
});

fn main() {
	println!("CONFIG: {:?}", CONFIG);
	println!("*CONFIG: {:?}", *CONFIG);
	println!("*CONFIG: {:?}", CONFIG.get(&"key".to_string()));
}
