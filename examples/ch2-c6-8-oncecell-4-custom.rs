use once_cell::sync::Lazy;

struct Config {
	database_url: String,
	max_connections: usize,
}

impl Config {
	fn new() -> Self {
		Self {
			database_url: "postgres://localhost".to_string(),
			max_connections: 10,
		}
	}
}

// Create a lazily initialized static variable
static CONFIG: Lazy<Config> = Lazy::new(|| Config::new());

fn main() {
	println!("Database URL: {}", CONFIG.database_url);
	println!("Max Connections: {}", CONFIG.max_connections);
}
