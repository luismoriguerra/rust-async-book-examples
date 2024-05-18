use futures_util::future::join_all;
use std::io::Write;
use std::{
	fs::{File, OpenOptions},
	sync::Arc,
};
use tokio::sync::Mutex;

fn get_file_handle(file_name: &dyn ToString) -> Arc<Mutex<File>> {
	let file: Result<File, std::io::Error> =
		OpenOptions::new().append(true).open(file_name.to_string());

	match file {
		Ok(opened_file) => Arc::new(Mutex::new(opened_file)),
		Err(_) => Arc::new(Mutex::new(File::create(file_name.to_string()).unwrap())),
	}
}

async fn write_log(
	file_arc_mutex: Arc<Mutex<File>>,
	username: String,
) -> Result<bool, String> {
	let mut guard = file_arc_mutex.lock().await;

	let lined_entry =
		format!("user: {}, time:{} \n", username, chrono::Local::now());

	match guard.write_all(lined_entry.as_bytes()) {
		Ok(_) => println!("written for: {}", username),
		Err(e) => println!("{}", e),
	}

	std::mem::drop(guard);
	Ok(true)
}

#[tokio::main]
async fn main() {
	let login_file_base_ref = get_file_handle(&"login_3.txt");

	let names = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
	let mut file_update_requests = Vec::new();

	for names in names {
		let login_file = login_file_base_ref.clone();

		let login_file_update_request = tokio::task::spawn(async move {
			write_log(login_file, names.to_string()).await
		});

		file_update_requests.push(login_file_update_request);
	}

	let _ = join_all(file_update_requests).await;
}
