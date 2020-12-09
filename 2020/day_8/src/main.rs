
mod vm;
use crate::vm::{VM, Ins};
use log::*;

use std::io::{stdout, Write};
use crossterm::{
    cursor::{MoveTo, Hide},
    execute,
    style::{style, Color, PrintStyledContent},
    terminal::{Clear, ClearType},
};

use std::{thread, time};

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

    // brute force, just run the machine with each Nop or Jmp swapped
    let mut replace_positions = vec![];
    for (i, val) in instructions.iter().enumerate() {
        let (ins,  _) = val;
        if *ins == Ins::Nop || *ins == Ins::Jmp {
            replace_positions.push(i);
        }
    }
    let replace_positions = replace_positions;

    for replace_pos in replace_positions {
        let mut instructions = instructions.clone();
        match instructions[replace_pos] {
            (Ins::Nop, a) => instructions[replace_pos] = (Ins::Jmp, a),
            (Ins::Jmp, a) => instructions[replace_pos] = (Ins::Nop, a),
            _ => unreachable!()
        }

        thread::sleep(time::Duration::from_millis(50));

        let vm = VM::new(instructions);

        execute!(
            stdout(),
            Clear(ClearType::All),
            Hide
        ).unwrap();

        draw_blob(replace_pos, Color::Green);

        if let Ok(result) = loop_detect(vm) {
            display_result(result);
            break;
        }
    };
}

// loop, return Result of last acc value if code exits,
// or Err of last acc value if loop detected
fn loop_detect(mut vm: VM) -> Result<i32, i32> {
    let mut instruction_history = vec![vm.pc()];

    while !vm.exited() {
        let acc = vm.acc();
        vm.step();
        let pc = vm.pc();
  
        thread::sleep(time::Duration::from_millis(1));

        draw_blob(pc, Color::Yellow);

        if let Some(pos) = instruction_history.iter().position(|&x| x == pc) {
            for pc in instruction_history.iter().skip(pos) {
                draw_blob(*pc, Color::Red);
            }
            return Err(acc);
        }

        instruction_history.push(pc);
        debug!("{:?}", instruction_history);
    }

    Ok(vm.acc())
}

fn draw_blob(pc: usize, colour: Color) -> () {
    execute!(
        stdout(),
        MoveTo((pc as u16) % 64, (pc / 64) as u16),
        PrintStyledContent(style('▓').with(colour))
    ).unwrap();
}

fn display_result(acc: i32) -> () {
    execute!(
        stdout(),
        MoveTo(0,10),
        PrintStyledContent(style(acc).with(Color::Green))
    ).unwrap();
}
