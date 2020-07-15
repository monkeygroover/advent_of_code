use log::*;

mod vm;
use crate::vm::{VM, State};

fn main() {
    env_logger::init();

    let initial_memory: Vec<i64> = include_str!("input.txt")
    .trim()
    .split(",")
    .map(|s| s.parse::<i64>().unwrap())
    .collect();

    let mut vm = VM::new(0, initial_memory);
    let mut command: Vec<i64> = vec![];
    loop {
        match vm.run() {
            State::InputPending => {
                if command.len() == 0 {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("Failed to read from stdin");
                
                    input.chars().rev().for_each(|char| {
                        let ascii = char as i64;
                        info!("{}", ascii);
                        command.push(ascii);
                    });
                }

                vm.handle_input(command.pop().unwrap());
            },
            State::OutputProduced(val) => {
                if val < 256 {
                    print!("{}", val as u8 as char);
                } else {
                    return ();
                }
            },
            State::Halt => {info!("Halt"); return ()},
            State::Continue => ()
        }
    }
}
