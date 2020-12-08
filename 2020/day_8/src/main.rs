
mod vm;
use crate::vm::{VM, Ins};
use log::*;

fn main() {
    env_logger::init();

    let instructions: Vec<(Ins, i32)> = include_str!("input.txt")
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

    let part1 = loop_detect(VM::new(instructions.clone()));

    // brute force, just run the machine with each Nop or Jmp swapped
    let mut replace_positions = vec![];
    for (i, val) in instructions.iter().enumerate() {
        let (ins,  _) = val;
        if *ins == Ins::Nop || *ins == Ins::Jmp {
            replace_positions.push(i);
        }
    }
    let replace_positions = replace_positions;

    let part2: Vec<i32> = replace_positions.iter().flat_map(|replace_pos| {
        let mut instructions = instructions.clone();
        match instructions[*replace_pos] {
            (Ins::Nop, a) => instructions[*replace_pos] = (Ins::Jmp, a),
            (Ins::Jmp, a) => instructions[*replace_pos] = (Ins::Nop, a),
            _ => unreachable!()
        }

        let vm = VM::new(instructions);

        match loop_detect(vm) {
            Ok(result) => vec![result],
            Err(_) => vec![]
        }
    }).collect();

    println!("{:?}, {:?}", part1, part2);
}

// loop, return Result of last acc value if code exits,
// or Err of last acc value if loop detected
fn loop_detect(mut vm: VM) -> Result<i32, i32> {
    let mut instruction_history = vec![vm.pc()];

    while !vm.exited() {
        let acc = vm.acc();
        vm.step();
        let pc = vm.pc();
  
        if instruction_history.contains(&pc) {
            return Err(acc);
        }
        instruction_history.push(pc);
        debug!("{:?}", instruction_history);
    }

    Ok(vm.acc())
}
