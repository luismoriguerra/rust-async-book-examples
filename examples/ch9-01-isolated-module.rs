use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::runtime::{Builder, Runtime};
use tokio::task::JoinHandle;

pub type AddFutMap = Lazy<Arc<Mutex<HashMap<String, JoinHandle<i32>>>>>;

static TOKIO_RUNTIME: Lazy<Runtime> = Lazy::new(|| {
	let r = Builder::new_multi_thread()
		.enable_all()
		.build()
		.expect("Failed to create Tokio runtime");

	r
});

async fn async_add(a: i32, b: i32) -> i32 {
	println!("starting async_add");
	tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
	println!("finished async_add");
	a + b
}

fn add_handler(
	a: Option<i32>,
	b: Option<i32>,
	id: Option<String>,
) -> Result<(Option<i32>, Option<String>), String> {
	//static ensures that MAP is available for the entire duration of the program, making it a global variable.
	static MAP: AddFutMap = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));
	match (a, b, id) {
		(Some(a), Some(b), None) => {
			let handle = TOKIO_RUNTIME.spawn(async_add(a, b));
			let id = uuid::Uuid::new_v4().to_string();
			MAP.lock().unwrap().insert(id.clone(), handle);
			Ok((None, Some(id)))
		}
		(None, None, Some(id)) => {
			let handle = match MAP.lock().unwrap().remove(&id) {
				Some(handle) => handle,
				None => return Err("No handle found".to_string()),
			};
			let result: i32 = match TOKIO_RUNTIME.block_on(async { handle.await }) {
				Ok(result) => result,
				Err(e) => return Err(e.to_string()),
			};
			Ok((Some(result), None))
		}
		_ => Err("either a or b need to be provided or a
            handle_id"
			.to_string()),
	}
}

pub fn send_add(a: i32, b: i32) -> Result<String, String> {
	match add_handler(Some(a), Some(b), None) {
		Ok((None, Some(id))) => Ok(id),
		Ok(_) => Err("Something went wrong, please contact author".to_string()),
		Err(e) => Err(e),
	}
}

pub fn get_add(id: String) -> Result<i32, String> {
	match add_handler(None, None, Some(id)) {
		Ok((Some(result), None)) => Ok(result),
		Ok(_) => Err("Something went wrong, please contact author".to_string()),
		Err(e) => Err(e),
	}
}

fn main() {
	println!("Hello, world!");
	let id = send_add(1, 2).unwrap();
	println!("id: {}", id);
	std::thread::sleep(std::time::Duration::from_secs(4));
	println!("main sleep done");
	let result = get_add(id).unwrap();
	println!("result: {}", result);
}
