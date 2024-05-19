// Importing necessary modules from the Rust standard library and tokio crate
use std::future::Future; // For defining asynchronous tasks
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering}; // For atomic operations
use tokio::task::JoinHandle; // For handling spawned asynchronous tasks

// Defining static atomic variables
static OPEN: AtomicBool = AtomicBool::new(false); // A flag to control the circuit state
static COUNT: AtomicUsize = AtomicUsize::new(0); // A counter to track the number of error tasks

// Function to spawn an asynchronous task
fn spawn_task<F, T>(future: F) -> Result<JoinHandle<T>, String>
where
	F: Future<Output = T> + Send + 'static, // F must implement Future, be Send, and have a 'static lifetime
	T: Send + 'static,                      // T must be Send and have a 'static lifetime
{
	let open = OPEN.load(Ordering::SeqCst); // Load the value of OPEN atomically with sequential consistency
	if open == false {
		// Check if the circuit is closed
		return Ok(tokio::task::spawn(future)); // Spawn the task if the circuit is closed
	}
	Err("Circuit Open".to_string()) // Return an error if the circuit is open
}

// Asynchronous function to simulate an error task
async fn error_task() {
	println!("error task running");
	let count = COUNT.fetch_add(1, Ordering::SeqCst); // Atomically increment the COUNT
	if count == 2 {
		// If the count reaches 2
		println!("opening circuit");
		OPEN.store(true, Ordering::SeqCst); // Open the circuit
	}
}

// Asynchronous function to simulate a passing task
async fn passing_task() {
	println!("passing task running");
}

// Main asynchronous function
#[tokio::main] // Macro to mark the main function as asynchronous using Tokio
async fn main() -> Result<(), String> {
	let _ = spawn_task(passing_task())?.await; // Spawn and await the passing task
	let _ = spawn_task(error_task())?.await; // Spawn and await the first error task
	let _ = spawn_task(error_task())?.await; // Spawn and await the second error task
	let _ = spawn_task(error_task())?.await; // Spawn and await the third error task
	let _ = spawn_task(passing_task())?.await; // Attempt to spawn and await another passing task
	Ok(()) // Return Ok if all tasks complete successfully
}
 