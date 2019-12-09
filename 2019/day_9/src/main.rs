use std::thread;

mod vm;

use crate::vm::{VM, Input};

fn main() {
    env_logger::init();

    let mut initial_memory: Vec<i64> = include_str!("input.txt")
    .trim()
    .split(",")
    .map(|s| s.parse::<i64>().unwrap())
    .collect();

    initial_memory.resize(1048576, 0);

    calculate_value(initial_memory);
}

fn calculate_value(initial_memory: Vec<i64>) -> () {
    let (mut amp_a, sender_a) = VM::new("A", initial_memory);

    let thread_a = thread::spawn(move || {
        amp_a.run(None)
    });

    let _z = sender_a.send(Input(2));

    let _f = thread_a.join();
}
