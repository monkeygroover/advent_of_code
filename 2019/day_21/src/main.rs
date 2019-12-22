use log::*;

mod vm;
use crate::vm::{VM, State};

fn main() {
    env_logger::init();

    let mut initial_memory: Vec<i64> = include_str!("input.txt")
    .trim()
    .split(",")
    .map(|s| s.parse::<i64>().unwrap())
    .collect();

    initial_memory.resize(1048576, 0);

    let script1 =
"NOT A J
NOT C T
AND D T
OR T J
WALK
";

    println!("part1 damage {:?}", run_springscript(script1, initial_memory.clone()));

    let script2 =
"NOT A J
AND D J
NOT B T
AND D T
AND H T
OR T J
NOT C T
AND D T
AND E T
OR T J
NOT C T
AND D T
AND H T
OR T J
RUN
";

    println!("part2 damage {:?}", run_springscript(script2, initial_memory.clone()));
}

fn run_springscript(script: &str, initial_memory: Vec<i64>) -> Option<i64> {
    let mut bot = VM::new("Robot", initial_memory);

    let mut script_iter = script.chars();
    loop {
        match bot.run() {
            State::InputPending => {
                match script_iter.next() {
                    Some(value) => {
                        let next_input = value as i64;
                        info!("=> {}", next_input);
                        bot.handle_input(next_input);
                    },
                    None => panic!()
                }
            },
            State::OutputProduced(val) => {
                if val < 256 {
                    print!("{}", val as u8 as char);
                } else {
                    return Some(val);
                }
            },
            State::Halt => {info!("Halt"); return None},
            State::Continue => ()
        }
    }
}
