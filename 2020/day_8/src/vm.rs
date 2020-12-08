use log::*;

#[derive(Debug, Copy, Clone)]
pub enum Ins {
    Nop,
    Acc,
    Jmp
}

pub struct VM{
    memory: Vec<(Ins, i32)>,
    pc: usize, 
    acc: i32
}

impl VM {
    pub fn new(initial_memory: Vec<(Ins, i32)>) -> VM {
        VM{
            memory: initial_memory,
            pc: 0,
            acc: 0}
    }

    pub fn step(&mut self) -> () {
        let (ins, val) = self.get_instruction();

        match ins {
            Ins::Nop => self.do_nop(),
            Ins::Acc => self.do_acc(val),
            Ins::Jmp => self.do_jmp(val)
        };
    }

    pub fn acc(&self) -> i32 {
        self.acc
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    fn get_instruction(&mut self) -> (Ins, i32) {
        self.memory[self.pc]
     }

     fn do_nop(&mut self) -> () {
        self.pc += 1;
        debug!("nop \tpc={} acc={}", self.pc, self.acc);
    }

    fn do_acc(&mut self, val: i32) -> () {
        self.acc += val;
        self.pc += 1;
        debug!("acc {} \tpc={} acc={}", val, self.pc, self.acc);
    }

    fn do_jmp(&mut self, val: i32) -> () {
        if val < 0 {
            self.pc -= val.abs() as usize
        } else {
            self.pc += val as usize;
        }
        debug!("jmp {} \tpc={} acc={}", val, self.pc, self.acc);
    }
}
