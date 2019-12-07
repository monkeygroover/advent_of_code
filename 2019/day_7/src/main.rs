use itertools::Itertools;
use std::thread;

mod vm;

use crate::vm::{VM, Input};

fn main() {
    env_logger::init();

    let initial_memory: Vec<i32> = include_str!("input.txt")
    .trim()
    .split(",")
    .map(|s| s.parse::<i32>().unwrap())
    .collect();

    let answer = (5..=9).permutations(5).map(|combination| {
        calculate_value(initial_memory.clone(), combination)
    }).max();

    println!("part2 {:?}", answer.unwrap());
}

fn calculate_value(initial_memory: Vec<i32>, phases: Vec<i32>) -> i32 {
    let (mut amp_a, sender_a) = VM::new("A", initial_memory.clone());
    let (mut amp_b, sender_b) = VM::new("B", initial_memory.clone());
    let (mut amp_c, sender_c) = VM::new("C", initial_memory.clone());
    let (mut amp_d, sender_d) = VM::new("D", initial_memory.clone());
    let (mut amp_e, sender_e) = VM::new("E", initial_memory.clone());

    let clone_b = sender_b.clone();
    let thread_a = thread::spawn(move || {
        amp_a.run(clone_b)
    });

    let clone_c = sender_c.clone();
    let thread_b = thread::spawn(move || {
        amp_b.run(clone_c)
    });

    let clone_d = sender_d.clone();
    let thread_c = thread::spawn(move || {
        amp_c.run(clone_d);
    });

    let clone_e = sender_e.clone();
    let thread_d = thread::spawn(move || {
        amp_d.run(clone_e);
    });

    let clone_a = sender_a.clone();
    let thread_e = thread::spawn(move || {
        amp_e.run(clone_a)
    });

    let _a = sender_a.send(Input(phases[0]));
    let _b = sender_b.send(Input(phases[1]));
    let _c = sender_c.send(Input(phases[2]));
    let _d = sender_d.send(Input(phases[3]));
    let _e = sender_e.send(Input(phases[4]));

    let _z = sender_a.send(Input(0));

    let _f = thread_a.join();
    let _g = thread_b.join();
    let _h = thread_c.join();
    let _i = thread_d.join();
    let thruster = thread_e.join();

    thruster.unwrap()
}
