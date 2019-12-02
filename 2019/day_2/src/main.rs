fn main() {
    let initial_memory: Vec<i32> = include_str!("input.txt")
                                .trim()
                                .split(",")
                                .map(|s| s.parse::<i32>().unwrap())
                                .collect();

    println!("part1 = {}", run(12,2, &mut initial_memory.clone()));

    for noun in 1..100 {
        for verb in 1..100 {
            if run(noun, verb, &mut initial_memory.clone()) == 19690720 {
                println!("part2 = {}", 100*noun+verb);
                return;
            }
        }
    }
}

fn run(noun: i32, verb: i32, memory: &mut Vec<i32>) -> i32 {
    memory[1] = noun;
    memory[2] = verb;

    let mut pc :usize = 0;
    while handle_op(pc, memory) {
        pc = pc + 4;
    }

    memory[0]
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
