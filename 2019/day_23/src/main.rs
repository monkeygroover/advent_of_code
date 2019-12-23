use log::*;

use std::collections::VecDeque;

mod vm;
use crate::vm::{VM, State};

fn main() {
    env_logger::init();

    let initial_memory: Vec<i64> = include_str!("input.txt")
    .trim()
    .split(",")
    .map(|s| s.parse::<i64>().unwrap())
    .collect();

    let mut nics = (0..50).map(|addr| NIC::boot(addr, initial_memory.clone())).collect::<Vec<NIC>>();

    loop {
        for i in 0..nics.len() {
            if let NICState::OutputProduced(addr, x, y) = nics[i].run_continutation() {
                if addr == 255 {
                    println!{"part1: {}", y};
                } else {
                    nics[addr as usize].queue_input(x);
                    nics[addr as usize].queue_input(y);
                }
            }
        }
    }
}

enum NICState {
    Idle,
    OutputProduced(i64, i64, i64),
    Halt
}


struct NIC {
    vm: VM,
    input_q : VecDeque<i64>,
    output_acc: Vec<i64>
}

impl NIC {
    fn boot(address: i64, initial_memory: Vec<i64>) -> NIC {
        let mut vm = VM::new(address, initial_memory);
        assert!(vm.run() == State::InputPending);
        assert!(vm.handle_input(address) == State::Continue);
        NIC{vm: vm,
            input_q: VecDeque::new(),
            output_acc: vec![]}
    }

    fn queue_input(&mut self, input: i64) -> () {
        self.input_q.push_back(input);
    }

    fn run_continutation(&mut self) -> NICState {
        match self.vm.run() {
            State::InputPending => {
                match self.input_q.pop_front() {
                    Some(value) => {
                        info!("=> {}", value);
                        self.vm.handle_input(value);
                    },
                    None => {
                        self.vm.handle_input(-1);
                    }
                }
                NICState::Idle
            },
            State::OutputProduced(val) => {
                self.output_acc.push(val);

                let result: NICState;
                if self.output_acc.len() == 3 {
                    info!("output, triple => {:?}", self.output_acc);

                    result = NICState::OutputProduced(self.output_acc[0], self.output_acc[1], self.output_acc[2]);
                    self.output_acc.clear();
                } else {
                    result = NICState::Idle;
                }

                result
            },
            State::Halt => NICState::Halt,
            State::Continue => NICState::Idle
        }
    }
}
