use log::*;
use std::cell::RefCell;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

#[derive(PartialEq)]
enum State {
    Continue,
    Halt
}

#[derive(Copy, Clone)]
pub struct Input(pub i32);

pub struct VM{
    name: String,
    memory: RefCell<Vec<i32>>,
    pc: usize,
    receiver: Receiver<Input>,
    output: Option<Sender<Input>>,
    last_output: i32
}

impl VM {
    pub fn new(name: &str, initial_memory: Vec<i32>) -> (VM, Sender<Input>) {
        let (sender, receiver) = mpsc::channel();

        let vm = VM{name: name.to_string(),
           memory: RefCell::new(initial_memory),
           pc: 0,
           receiver: receiver,
           output: None,
           last_output: 0
           };

           (vm, sender.clone())
    }

    pub fn run(&mut self, output: Sender<Input>) -> i32 {
        self.output = Some(output);

        while self.handle_op() != State::Halt {
                debug!("{} continue", self.name);
        }

        debug!("{} HALTING", self.name);
        self.last_output
    }

    fn handle_op(&mut self) -> State {
        let raw_op = self.memory.borrow_mut()[self.pc];
        let op = raw_op % 100;
        let mode_list = vec![(raw_op/100) % 10, (raw_op/1000) % 10, (raw_op/10000) % 10];

        match op {
            99 => State::Halt,
            1 => self.add(mode_list),
            2 => self.multiply(mode_list),
            3 => self.read(),
            4 => self.write(mode_list),
            5 => self.jump_if_true(mode_list),
            6 => self.jump_if_false(mode_list),
            7 => self.less_than(mode_list),
            8 => self.equals(mode_list),
            _ => panic!("bad op code")
        }
    }

    fn get_param(&mut self, pos: usize, modes: &Vec<i32>) -> i32 {
        let mem = self.memory.borrow_mut();
        let param = mem[self.pc + pos as usize];
        match modes[pos - 1] {
            0 => mem[param as usize],
            1 => param,
            _ => panic!("bad mode")
        }
    }

    fn add(&mut self, modes: Vec<i32>) -> State {
        let a = self.get_param(1, &modes);
        let b = self.get_param(2, &modes);
        let mut mem = self.memory.borrow_mut();
        let dest = mem[self.pc+3];
        mem[dest as usize] = a + b;
        debug!("{} [{}] <- {} + {}", self.name, dest, a, b);
        self.pc = self.pc + 4;
        State::Continue
    }

    fn multiply(&mut self, modes: Vec<i32>) -> State {
        let a = self.get_param(1, &modes);
        let b = self.get_param(2, &modes);
        let mut mem = self.memory.borrow_mut();
        let dest = mem[self.pc+3];
        mem[dest as usize] = a * b;
        debug!("{} [{}] <- {} * {}", self.name, dest, a, b);
        self.pc = self.pc + 4;
        State::Continue
    }

    fn jump_if_true(&mut self, modes: Vec<i32>) -> State {
        let a = self.get_param(1, &modes);
        let b = self.get_param(2, &modes);
        if a != 0 {
            self.pc = b as usize
        } else {
            self.pc = self.pc + 3;
        }
        State::Continue
    }

    fn jump_if_false(&mut self, modes: Vec<i32>) -> State {
        let a = self.get_param(1, &modes);
        let b = self.get_param(2, &modes);
        if a == 0 {
            self.pc = b as usize
        } else {
            self.pc = self.pc + 3;
        }
        State::Continue
    }

    fn less_than(&mut self, modes: Vec<i32>) -> State {
        let a = self.get_param(1, &modes);
        let b = self.get_param(2, &modes);
        let mut mem = self.memory.borrow_mut();
        let dest = mem[self.pc+3];
        mem[dest as usize] = if a < b {1} else {0};
        self.pc = self.pc + 4;
        State::Continue
    }

    fn equals(&mut self, modes: Vec<i32>) -> State {
        let a = self.get_param(1, &modes);
        let b = self.get_param(2, &modes);
        let mut mem = self.memory.borrow_mut();
        let dest = mem[self.pc+3];
        mem[dest as usize] = if a == b {1} else {0};
        self.pc = self.pc + 4;
        State::Continue
    }


    fn read(&mut self) -> State {
        let mut mem = self.memory.borrow_mut();
        let dest = mem[self.pc+1];
        debug!("{} read pending for [{}]", self.name, dest);

        match self.receiver.recv() {
               Ok(Input(value)) => {
                debug!("{} got {} for [{}]", self.name, value, dest);
                mem[dest as usize] = value;
               }
               Err(_) => panic!("received junk")
           }

        self.pc = self.pc + 2;
        State::Continue
    }

    fn write(&mut self, modes: Vec<i32>) -> State {
        let mem = self.memory.borrow_mut();
        let out = match modes[0] {
            0 => mem[mem[self.pc+1 as usize] as usize],
            1 => mem[self.pc+1 as usize],
            _ => panic!("bad mode")
        };

        //send this to next amp
        info!("{} write {}", self.name, out);
        self.last_output = out;
        let _res = self.output.clone().unwrap().send(Input(out));

        self.pc = self.pc + 2;
        State::Continue
    }
}
