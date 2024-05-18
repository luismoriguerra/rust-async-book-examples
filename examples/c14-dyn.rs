use std::{
	fs::{File, OpenOptions},
	sync::{Arc, Mutex},
	thread::JoinHandle,
};

fn print_string(s: &dyn ToString) {
	println!("{}", s.to_string());
}

struct Person {
	name: String,
	age: u8,
}

impl ToString for Person {
	fn to_string(&self) -> String {
		format!("{} is {} years old", self.name, self.age)
	}
}

fn print_person(p: &dyn ToString) {
	println!("{}", p.to_string());
}

enum Color {
	Red,
	Green,
	Blue,
}

impl ToString for Color {
	fn to_string(&self) -> String {
		match self {
			Color::Red => "Red".to_string(),
			Color::Green => "Green".to_string(),
			Color::Blue => "Blue".to_string(),
		}
	}
}

fn print_color(c: &dyn ToString) {
	println!("{}", c.to_string());
}

fn print_as_string(value: &dyn ToString) {
	println!("{}", value.to_string());
}

type AsyncFileHandle = Arc<Mutex<File>>;
type FileJoinHandle = JoinHandle<Result<bool, String>>;

fn get_handle(file_path: &dyn ToString) -> AsyncFileHandle {
	match OpenOptions::new().append(true).open(file_path.to_string()) {
		Ok(opened_file) => Arc::new(Mutex::new(opened_file)),
		Err(_) => Arc::new(Mutex::new(File::create(file_path.to_string()).unwrap())),
	}
}

fn main() {
	let s = "Hello, world!";
	print_string(&s); // Output: Hello, world!

	let person = Person {
		name: "Alice".to_string(),
		age: 30,
	};
	print_person(&person); // Output: Alice is 30 years old

	let color = Color::Green;
	print_color(&color); // Output: Green

	let my_string = "Hello, world!";
	let my_number = 42;

	print_as_string(&my_string); // &str implements ToString

	// print_as_string(&my_number); // Error needs to implement ToString

	let file_handle = get_handle(&"file.txt");
	let file_name = String::from("file.txt");
	let file_handle = get_handle(&file_name);
}
