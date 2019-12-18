use log::*;

mod vm;
use crate::vm::{VM, State};

fn main() {
    let mut initial_memory: Vec<i64> = include_str!("input.txt")
    .trim()
    .split(",")
    .map(|s| s.parse::<i64>().unwrap())
    .collect();

    initial_memory.resize(1048576, 0);
    initial_memory[0] = 2;

    let mut bot = VM::new("Robot", initial_memory.clone());

    loop {
        match bot.run() {
            State::OutputProduced(val) => {
                print!("{}", val as u8 as char);
            },
            State::InputPending => {
            },
            State::Halt => break,
            State::Continue => ()
        }
    }

    let code =
"A,A,C,B,C,B,C,B,A,B
R,6,L,8,R,8
L,8,R,6,L,10,L,10
R,4,R,6,R,6,R,4,R,4
n
";

    let mut bot2 = VM::new("Robot", initial_memory.clone());

    let mut code_iter = code.chars();
    loop {
        match bot2.run() {
            State::OutputProduced(val) => {
                if val < 256 {
                    print!("{}", val as u8 as char);
                } else {
                    print!("part 2 {}", val);
                }
            },
            State::InputPending => {
                match code_iter.next() {
                    Some(value) => {
                        let next_input = value as i64;
                        info!("=> {}", next_input);
                        bot2.handle_input(next_input);
                    },
                    None => panic!()
                }
            },
            State::Halt => break,
            State::Continue => ()
        }
    }
}
