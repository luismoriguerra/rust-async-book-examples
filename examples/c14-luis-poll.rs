use futures_util::{future::join_all, Future};
use std::io::Write;
use std::task::{Context, Poll};
use std::{
	fs::{File, OpenOptions},
	pin::Pin,
	sync::{Arc, Mutex},
};
use tokio::task::JoinHandle;

fn get_file_handle(file_name: &dyn ToString) -> Arc<Mutex<File>> {
	let file: Result<File, std::io::Error> =
		OpenOptions::new().append(true).open(file_name.to_string());

	match file {
		Ok(opened_file) => Arc::new(Mutex::new(opened_file)),
		Err(_) => Arc::new(Mutex::new(File::create(file_name.to_string()).unwrap())),
	}
}

struct AsyncWriteFuture {
	file_arc_mutex: Arc<Mutex<File>>,
	entry: String,
}

impl Future for AsyncWriteFuture {
	type Output = Result<bool, String>;

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		let mut guard = match self.file_arc_mutex.try_lock() {
			Ok(guard) => guard,
			Err(error) => {
				println!("error for {} : {}", self.entry, error);
				cx.waker().wake_by_ref();
				return Poll::Pending;
			}
		};

		let lined_entry =
			format!("user: {}, time:{} \n", self.entry, chrono::Local::now());
		match guard.write_all(lined_entry.as_bytes()) {
			Ok(_) => println!("written for: {}", self.entry),
			Err(e) => println!("{}", e),
		}

		Poll::Ready(Ok(true))
	}
}

fn write_log(
	file_arc_mutex: Arc<Mutex<File>>,
	username: String,
) -> JoinHandle<Result<bool, String>> {
	let future_request = AsyncWriteFuture {
		file_arc_mutex,
		entry: username,
	};

	tokio::task::spawn(async move { future_request.await })
}

#[tokio::main]
async fn main() {
	let login_file_base_ref = get_file_handle(&"login_luis.txt");

	let names = ["luis", "jose", "pedro", "juan", "maria", "josefa"];
	let mut file_update_requests = Vec::new();

	for names in names {
		let login_file = login_file_base_ref.clone();

		let login_file_update_request = write_log(login_file, names.to_string());

		file_update_requests.push(login_file_update_request);
	}

	let _ = join_all(file_update_requests).await;
}
