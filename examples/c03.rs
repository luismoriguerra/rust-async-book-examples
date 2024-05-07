use std::env;
use std::process::{exit, Command};
use std::thread::sleep;
use std::time::{Duration, Instant};

fn task(arg: usize) {
	println!("Running task... {}", arg);
	sleep(Duration::from_secs(1));
}
fn run_processes() {
	let mut process1 = Command::new(env::current_exe().unwrap())
		.arg("task")
		.arg("1")
		.spawn()
		.expect("Failed to start process1");

	let mut process2 = Command::new(env::current_exe().unwrap())
		.arg("task")
		.arg("6")
		.spawn()
		.expect("Failed to start process2");

	process1.wait().expect("Failed to wait for process1");
	process2.wait().expect("Failed to wait for process2");

	println!("Both processes have completed.");
}
fn main() {
	let args: Vec<String> = env::args().collect();

	let start = Instant::now();

	if args.len() > 2 && args[1] == "task" {
		let start_task_number = args[2].parse::<usize>().unwrap();
		task(start_task_number);
	} else {
		run_processes();
	}

	if args.len() <= 1 {
		let elapsed = start.elapsed();
		println!("The whole program took: {:?}", elapsed);
	}
}
