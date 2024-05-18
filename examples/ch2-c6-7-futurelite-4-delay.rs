use futures_lite::{future, Future};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct Delay {
	when: Instant,
}

impl Delay {
	fn new(dur: Duration) -> Self {
		Delay {
			when: Instant::now() + dur,
		}
	}
}

impl Future for Delay {
	type Output = ();

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		if Instant::now() >= self.when {
			Poll::Ready(())
		} else {
			cx.waker().wake_by_ref();
			Poll::Pending
		}
	}
}

async fn custom_delay_example() {
	let delay = Delay::new(Duration::from_secs(2));
	delay.await;
	println!("Custom delay completed");
}

fn main() {
	future::block_on(custom_delay_example());
}
