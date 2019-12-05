static INPUT: i32 = 5;

fn main() {
    let initial_memory: Vec<i32> = include_str!("input.txt")
                                .trim()
                                .split(",")
                                .map(|s| s.parse::<i32>().unwrap())
                                .collect();

    run(&mut initial_memory.clone());
}

fn run(memory: &mut Vec<i32>) -> i32 {
    let mut pc :usize = 0;
    while let Some(new_pc) = handle_op(pc, memory) {
        pc = new_pc;
    }

    memory[0]
}

fn handle_op(pc: usize, memory: &mut Vec<i32>) -> Option<usize> {
    // split op code
    let op = memory[pc] % 100;
    let mode_list = vec![(memory[pc]/100) % 10, (memory[pc]/1000) % 10, (memory[pc]/10000) % 10];

    //println!("op {:?}, modes: {:?}", op, mode_list);

    match op {
        99 => None,
        1 => add(pc, memory, mode_list),
        2 => multiply(pc, memory, mode_list),
        3 => store(pc, memory, mode_list),
        4 => output(pc, memory, mode_list),
        5 => jump_if_true(pc, memory, mode_list),
        6 => jump_if_false(pc, memory, mode_list),
        7 => less_than(pc, memory, mode_list),
        8 => equals(pc, memory, mode_list),
        _ => panic!("bad op code")
    }
}

fn add(pc: usize, memory: &mut Vec<i32>, modes: Vec<i32>) -> Option<usize> {
    //println!("memory {:?}, pc {}", memory, pc);
    let param1 = memory[pc+1 as usize];
    //println!("param1 {}", param1);
    let a = match modes[0] {
        0 => memory[param1 as usize],
        1 => param1,
        _ => panic!("bad mode")
    };

    let param2 = memory[pc+2 as usize];
    //println!("param2 {}", param2);
    let b = match modes[1] {
        0 => memory[param2 as usize],
        1 => param2,
        _ => panic!("bad mode")
    };

    let dest = memory[pc+3];

    memory[dest as usize] = a + b;
    Some(pc + 4)
}

fn multiply(pc: usize, memory: &mut Vec<i32>, modes: Vec<i32>) -> Option<usize> {
    let param1 = memory[pc+1 as usize];
    let a = match modes[0] {
        0 => memory[param1 as usize],
        1 => param1,
        _ => panic!("bad mode")
    };

    let param2 = memory[pc+2 as usize];
    let b = match modes[1] {
        0 => memory[param2 as usize],
        1 => param2,
        _ => panic!("bad mode")
    };

    let dest = memory[pc+3];

    memory[dest as usize] = a * b;
    Some(pc + 4)
}

fn store(pc: usize, memory: &mut Vec<i32>, _modes: Vec<i32>) -> Option<usize> {
    let dest = memory[pc+3];

    memory[dest as usize] = INPUT;
    Some(pc + 2)
}

fn output(pc: usize, memory: &mut Vec<i32>, modes: Vec<i32>) -> Option<usize> {
    let out = match modes[0] {
        0 => memory[memory[pc+1 as usize] as usize],
        1 => memory[pc+1 as usize],
        _ => panic!("bad mode")
    };

    println!("output {}", out);
    Some(pc + 2)
}

fn jump_if_true(pc: usize, memory: &mut Vec<i32>, modes: Vec<i32>) -> Option<usize> {
    let param1 = memory[pc+1 as usize];
    let a = match modes[0] {
        0 => memory[param1 as usize],
        1 => param1,
        _ => panic!("bad mode")
    };

    let param2 = memory[pc+2 as usize];
    let b = match modes[1] {
        0 => memory[param2 as usize],
        1 => param2,
        _ => panic!("bad mode")
    };

    if a != 0 {
        Some(b as usize)
    } else {
        Some(pc+3)
    }
}

fn jump_if_false(pc: usize, memory: &mut Vec<i32>, modes: Vec<i32>) -> Option<usize> {
    let param1 = memory[pc+1 as usize];
    let a = match modes[0] {
        0 => memory[param1 as usize],
        1 => param1,
        _ => panic!("bad mode")
    };

    let param2 = memory[pc+2 as usize];
    let b = match modes[1] {
        0 => memory[param2 as usize],
        1 => param2,
        _ => panic!("bad mode")
    };

    if a == 0 {
        Some(b as usize)
    } else {
        Some(pc + 3)
    }
}

fn less_than(pc: usize, memory: &mut Vec<i32>, modes: Vec<i32>) -> Option<usize> {
    let param1 = memory[pc+1 as usize];
    let a = match modes[0] {
        0 => memory[param1 as usize],
        1 => param1,
        _ => panic!("bad mode")
    };

    let param2 = memory[pc+2 as usize];
    let b = match modes[1] {
        0 => memory[param2 as usize],
        1 => param2,
        _ => panic!("bad mode")
    };

    let dest = memory[pc+3];

    if a < b {
        memory[dest as usize] = 1;
    } else {
        memory[dest as usize] = 0;
    }

    Some(pc + 4)
}

fn equals(pc: usize, memory: &mut Vec<i32>, modes: Vec<i32>) -> Option<usize> {
    let param1 = memory[pc+1 as usize];
    let a = match modes[0] {
        0 => memory[param1 as usize],
        1 => param1,
        _ => panic!("bad mode")
    };

    let param2 = memory[pc+2 as usize];
    let b = match modes[1] {
        0 => memory[param2 as usize],
        1 => param2,
        _ => panic!("bad mode")
    };

    let dest = memory[pc+3];

    if a == b {
        memory[dest as usize] = 1;
    } else {
        memory[dest as usize] = 0;
    }

    Some(pc + 4)
}
