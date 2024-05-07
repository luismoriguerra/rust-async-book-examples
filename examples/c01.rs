use rust_async_book_examples::variab;

fn fibonaci(n: i32) -> i32 {
	if n <= 1 {
		return n;
	}
	return fibonaci(n - 1) + fibonaci(n - 2);
}

fn main() {
	let mut threads = vec![];

	for i in 0..10 {
		threads.push(std::thread::spawn(move || {
			println!("Fibonaci of {} is {}", i, fibonaci(i));
		}));
	}

	for t in threads {
		t.join().unwrap();
	}
}
