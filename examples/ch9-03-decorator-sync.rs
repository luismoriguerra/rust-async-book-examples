trait Greeting {
	fn greet(&self) -> String;
}

struct HelloWorld;

impl Greeting for HelloWorld {
	fn greet(&self) -> String {
		"Hello, World!".to_string()
	}
}

struct ExcitedGreeting<T: Greeting> {
	inner: T,
}

impl<T: Greeting> Greeting for ExcitedGreeting<T> {
	fn greet(&self) -> String {
		let mut greeting = self.inner.greet();
		greeting.push_str(" I'm so excited to be in Rust!");
		greeting
	}
}

// fn main() {
// 	let raw_one = HelloWorld;
// 	let raw_two = HelloWorld;
// 	let decorated = ExcitedGreeting { inner: raw_two };
// 	println!("{}", raw_one.greet());
// 	println!("{}", decorated.greet());
// }

fn main() {
	#[cfg(feature = "logging_decorator")]
	let hello = ExcitedGreeting { inner: HelloWorld };

	#[cfg(not(feature = "logging_decorator"))]
	let hello = HelloWorld;

	println!("{}", hello.greet());
}

//cargo run --features "logging_decorator"
