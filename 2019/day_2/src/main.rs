fn main() {
    let mut memory: Vec<i32> = include_str!("input.txt")
                                .trim()
                                .split(",")
                                .map(|s| s.parse::<i32>().unwrap())
                                .collect();

    memory[1] = 12;
    memory[2] = 2;

    let mut pc :usize = 0;
    while handle_op(pc, &mut memory) {
        pc = pc + 4;
    }

    print!("{}", memory[0]);
}

fn handle_op(pc: usize, memory: &mut Vec<i32>) -> bool {
    match memory[pc] {
        99 => false,
        1 => add(pc, memory),
        2 => multiply(pc, memory),
        _ => panic!("bullshit op code")
    }
}

fn add(pc: usize, memory: &mut Vec<i32>) -> bool {
    let a = memory[memory[pc+1] as usize];
    let b = memory[memory[pc+2] as usize];
    let dest = memory[pc+3] as usize;

    //println!("adding {} and {} and writing to {}", a, b, dest);

    memory[dest] = a + b;
    true
}

fn multiply(pc: usize, state: &mut Vec<i32>) -> bool {
    let a = state[state[pc+1] as usize];
    let b = state[state[pc+2] as usize];
    let dest = state[pc+3] as usize;

    //println!("multiplying {} and {}", a, b);

    state[dest] = a * b;
    true
}
