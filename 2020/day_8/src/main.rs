
mod vm;
use crate::vm::{VM, Ins};
use log::*;

fn main() {
    env_logger::init();

    let mut instructions: Vec<(Ins, i32)> = include_str!("input.txt")
    .trim()
    .lines()
    .map(|s| {
        let parts: Vec<&str> = s.split(" ").collect();

        let ins = match parts[0] {
            "nop" => Ins::Nop,
            "acc" => Ins::Acc,
            "jmp" => Ins::Jmp,
            _ => unreachable!()
        };

        let val = parts[1].parse::<i32>().unwrap();

        (ins, val)
    })
    .collect();

    trace!("{:?}", instructions);

    let mut vm = VM::new(instructions);

    let mut instruction_history = vec![];

    loop {
        let pc = vm.pc();
        let acc = vm.acc();
        vm.step();
        if instruction_history.contains(&pc) {
            println!("{}", acc);
            break;
        }
        instruction_history.push(pc);
        debug!("{:?}", instruction_history);
    }
}
