static INPUT: i32 = 5;

fn main() {
    let mut initial_memory: Vec<i32> = include_str!("input.txt")
                                .trim()
                                .split(",")
                                .map(|s| s.parse::<i32>().unwrap())
                                .collect();

    run(&mut initial_memory);
}

fn run(memory: &mut Vec<i32>) -> () {
    let mut pc :usize = 0;
    while let Some(new_pc) = handle_op(pc, memory) {
        pc = new_pc;
    }
}

fn handle_op(pc: usize, memory: &mut Vec<i32>) -> Option<usize> {
    let op = memory[pc] % 100;
    let mode_list = vec![(memory[pc]/100) % 10, (memory[pc]/1000) % 10, (memory[pc]/10000) % 10];

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

fn get_param(pc: usize, memory: &mut Vec<i32>, pos: usize, modes: &Vec<i32>) -> i32 {
    let param = memory[pc + pos as usize];
    match modes[pos - 1] {
        0 => memory[param as usize],
        1 => param,
        _ => panic!("bad mode")
    }
}

fn add(pc: usize, memory: &mut Vec<i32>, modes: Vec<i32>) -> Option<usize> {
    let a = get_param(pc, memory, 1, &modes);
    let b = get_param(pc, memory, 2, &modes);
    let dest = memory[pc+3];
    memory[dest as usize] = a + b;
    Some(pc + 4)
}

fn multiply(pc: usize, memory: &mut Vec<i32>, modes: Vec<i32>) -> Option<usize> {
    let a = get_param(pc, memory, 1, &modes);
    let b = get_param(pc, memory, 2, &modes);
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
    let a = get_param(pc, memory, 1, &modes);
    let b = get_param(pc, memory, 2, &modes);
    if a != 0 { Some(b as usize) } else { Some(pc+3) }
}

fn jump_if_false(pc: usize, memory: &mut Vec<i32>, modes: Vec<i32>) -> Option<usize> {
    let a = get_param(pc, memory, 1, &modes);
    let b = get_param(pc, memory, 2, &modes);
    if a == 0 { Some(b as usize) } else { Some(pc + 3) }
}

fn less_than(pc: usize, memory: &mut Vec<i32>, modes: Vec<i32>) -> Option<usize> {
    let a = get_param(pc, memory, 1, &modes);
    let b = get_param(pc, memory, 2, &modes);
    let dest = memory[pc+3];
    memory[dest as usize] = if a < b {1} else {0};
    Some(pc + 4)
}

fn equals(pc: usize, memory: &mut Vec<i32>, modes: Vec<i32>) -> Option<usize> {
    let a = get_param(pc, memory, 1, &modes);
    let b = get_param(pc, memory, 2, &modes);
    let dest = memory[pc+3];
    memory[dest as usize] = if a == b {1} else {0};
    Some(pc + 4)
}
