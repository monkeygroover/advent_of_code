use log::*;

#[derive(PartialEq)]
pub enum State {
    Continue,
    InputPending,
    OutputProduced(i64),
    Halt
}

#[derive(Copy, Clone)]
pub struct Input(pub i64);

pub struct VM{
    name: String,
    memory: Vec<i64>,
    pc: i64,
    ix: i64,
    pending_input_param: i64,
    pending_input_mode: i64
}

impl VM {
    pub fn new(name: &str, initial_memory: Vec<i64>) -> VM {
        VM{ name: name.to_string(),
            memory: initial_memory,
            pc: 0,
            ix: 0,
            pending_input_param: 0,
            pending_input_mode: 0 }
    }

    pub fn run(&mut self) -> State {
        loop {
            let state = self.handle_op();
            if state != State::Continue { return state; }
        }
    }

    pub fn handle_input(&mut self, input: i64) -> State {
        self.set_memory(self.pending_input_param, input, self.pending_input_mode);
        State::Continue
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
            9 => self.set_ix(mode_list),
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
            2 => self.memory[(self.ix + address) as usize] = value,
            _ => panic!("bullshit")
        };
    }

    fn get_param(&mut self, pos: i64, modes: &Vec<i64>) -> i64 {
        let mode = modes[(pos - 1) as usize];
        let val = self.get_memory(self.pc + pos);

        let res = match mode {
            0 => self.get_memory(val),
            1 => val,
            2 => self.get_memory(self.ix + val),
            _ => panic!("bad mode")
        };

        trace!("get_param {} {:?} {}", res, mode, self.ix);

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
        trace!("{} read pending for [{}] {:?}", self.name, param, modes);
        self.pending_input_param = param;
        self.pending_input_mode = modes[0];
        self.pc = self.pc + 2;
        State::InputPending
    }

    fn write(&mut self, modes: Vec<i64>) -> State {
        let out = self.get_param(1, &modes);
        self.pc = self.pc + 2;
        State::OutputProduced(out)
    }

    fn set_ix(&mut self, modes: Vec<i64>) -> State {
        let a = self.get_param(1, &modes);
        let orig_base = self.ix;
        self.ix += a;
        debug!("{} ix {} -> {}", self.name, orig_base, self.ix);
        self.pc = self.pc + 2;
        State::Continue
    }
}
