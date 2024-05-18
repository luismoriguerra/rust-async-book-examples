use tokio::runtime::Runtime;

fn main() {
	// Create the runtime
	let rt = Runtime::new().unwrap();

	// Spawn a future onto the runtime
	rt.spawn(async {
		println!("now running on a worker thread");
	});
}
