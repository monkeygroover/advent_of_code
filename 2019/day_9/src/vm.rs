use log::*;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

#[derive(PartialEq)]
enum State {
    Continue,
    Halt
}

#[derive(Copy, Clone)]
pub struct Input(pub i64);

pub struct VM{
    name: String,
    memory: Vec<i64>,
    pc: i64,
    rel_base: i64,
    receiver: Receiver<Input>,
    output: Option<Sender<Input>>,
    last_output: i64
}

impl VM {
    pub fn new(name: &str, initial_memory: Vec<i64>) -> (VM, Sender<Input>) {
        let (sender, receiver) = mpsc::channel();

        let vm = VM{name: name.to_string(),
           memory: initial_memory,
           pc: 0,
           rel_base: 0,
           receiver: receiver,
           output: None,
           last_output: 0
           };

           (vm, sender.clone())
    }

    pub fn run(&mut self, output: Option<Sender<Input>>) -> i64 {
        self.output = output;

        while self.handle_op() != State::Halt {
                trace!("{} continue pc = {}", self.name, self.pc);
        }

        trace!("{} HALTING", self.name);
        self.last_output
    }

    fn handle_op(&mut self) -> State {
        let raw_op = self.get_memory(self.pc);

        let op = raw_op % 100;
        let mode_list = vec![(raw_op/100) % 10, (raw_op/1000) % 10, (raw_op/10000) % 10];

        match op {
            99 => State::Halt,
            1 => self.add(mode_list),
            2 => self.multiply(mode_list),
            3 => self.read(mode_list),
            4 => self.write(mode_list),
            5 => self.jump_if_true(mode_list),
            6 => self.jump_if_false(mode_list),
            7 => self.less_than(mode_list),
            8 => self.equals(mode_list),
            9 => self.set_rel_base(mode_list),
            x => panic!("bad op code {}", x)
        }
    }

    fn get_memory(&mut self, address: i64) -> i64 {
       if address < 0 {panic!("bad get address {}", address)}
       self.memory[address as usize]
    }

    fn set_memory(&mut self, address: i64, value: i64, mode: i64) -> () {
        match mode {
            0 => self.memory[address as usize] = value,
            2 => self.memory[(self.rel_base + address) as usize] = value,
            _ => panic!("bullshit")
        };
    }

    fn get_param(&mut self, pos: i64, modes: &Vec<i64>) -> i64 {
        let mode = modes[(pos - 1) as usize];
        let val = self.get_memory(self.pc + pos);

        let res = match mode {
            0 => self.get_memory(val),
            1 => val,
            2 => self.get_memory(self.rel_base + val),
            _ => panic!("bad mode")
        };

        trace!("get_param {} {:?} {}", res, mode, self.rel_base);

        res
    }

    fn add(&mut self, modes: Vec<i64>) -> State {
        let a = self.get_param(1, &modes);
        let b = self.get_param(2, &modes);
        let dest = self.get_memory(self.pc+3);
        self.set_memory(dest, a + b, modes[2]);
        debug!("{} [{}] <- {} + {}", self.name, dest, a, b);
        self.pc = self.pc + 4;
        State::Continue
    }

    fn multiply(&mut self, modes: Vec<i64>) -> State {
        let a = self.get_param(1, &modes);
        let b = self.get_param(2, &modes);
        let dest = self.get_memory(self.pc+3);
        self.set_memory(dest, a * b, modes[2]);
        debug!("{} [{}] <- {} * {}", self.name, dest, a, b);
        self.pc = self.pc + 4;
        State::Continue
    }

    fn jump_if_true(&mut self, modes: Vec<i64>) -> State {
        let a = self.get_param(1, &modes);
        let b = self.get_param(2, &modes);
        debug!("{} j_if_t {} {}", self.name, a, b);
        if a != 0 {
            self.pc = b
        } else {
            self.pc = self.pc + 3;
        }
        State::Continue
    }

    fn jump_if_false(&mut self, modes: Vec<i64>) -> State {
        let a = self.get_param(1, &modes);
        let b = self.get_param(2, &modes);
        debug!("{} j_if_f {} {}", self.name, a, b);
        if a == 0 {
            self.pc = b
        } else {
            self.pc = self.pc + 3;
        }
        State::Continue
    }

    fn less_than(&mut self, modes: Vec<i64>) -> State {
        let a = self.get_param(1, &modes);
        let b = self.get_param(2, &modes);
        let dest = self.get_memory(self.pc+3);
        debug!("{} [{}] <- {} < {}", self.name, dest, a, b);
        self.set_memory(dest,if a < b {1} else {0}, modes[2]);
        self.pc = self.pc + 4;
        State::Continue
    }

    fn equals(&mut self, modes: Vec<i64>) -> State {
        let a = self.get_param(1, &modes);
        let b = self.get_param(2, &modes);
        let dest = self.get_memory(self.pc+3);
        debug!("{} [{}] <- {} == {}", self.name, dest, a, b);
        self.set_memory(dest, if a == b {1} else {0}, modes[2]);
        self.pc = self.pc + 4;
        State::Continue
    }

    fn read(&mut self, modes: Vec<i64>) -> State {
        let param = self.get_memory(self.pc+1);
        debug!("{} read pending for [{}] {:?}", self.name, param, modes);

        match self.receiver.recv() {
            Ok(Input(value)) => {
                debug!("{} got {} for [{}]", self.name, value, param);
                self.set_memory(param ,value, modes[0]);
            }
            Err(_) => panic!("received junk")
        }

        self.pc = self.pc + 2;
        State::Continue
    }

    fn write(&mut self, modes: Vec<i64>) -> State {
        let out = self.get_param(1, &modes);

        //send this to next amp if set
        println!("{} -> {}", self.name, out);
        self.last_output = out;
        let _res = match self.output.clone() {
            Some(output) => output.send(Input(out)),
            None => Ok(())
        };

        self.pc = self.pc + 2;
        State::Continue
    }

    fn set_rel_base(&mut self, modes: Vec<i64>) -> State {
        let a = self.get_param(1, &modes);
        let orig_base = self.rel_base;
        self.rel_base += a;
        debug!("{} rel_base {} -> {}", self.name, orig_base, self.rel_base);
        self.pc = self.pc + 2;
        State::Continue
    }
}
