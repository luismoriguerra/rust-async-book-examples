use futures_lite::future;

async fn say_hello() {
	println!("Hello, world!");
}

fn main() {
	future::block_on(say_hello());
}
