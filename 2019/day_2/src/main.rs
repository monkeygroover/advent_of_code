use rayon::prelude::*;

fn main() {
    let initial_memory: Vec<usize> = include_str!("input.txt")
                                .trim()
                                .split(",")
                                .map(|s| s.parse::<usize>().unwrap())
                                .collect();

    println!("part1 = {}", run(12,2, &mut initial_memory.clone()));

    for noun in 1_usize..100 {
        (1_usize..100).into_par_iter().for_each( |verb| {
            if run(noun, verb, &mut initial_memory.clone()) == 19690720 {
                println!("part2 = {}", 100*noun+verb);
                return;
            }})
    }
}

fn run(noun: usize, verb: usize, memory: &mut Vec<usize>) -> usize {
    memory[1] = noun;
    memory[2] = verb;

    let mut pc :usize = 0;
    while handle_op(pc, memory) {
        pc = pc + 4;
    }

    memory[0]
}

fn handle_op(pc: usize, memory: &mut Vec<usize>) -> bool {
    match memory[pc] {
        99 => false,
        1 => add(pc, memory),
        2 => multiply(pc, memory),
        _ => panic!("bullshit op code")
    }
}

fn add(pc: usize, memory: &mut Vec<usize>) -> bool {
    let a = memory[memory[pc+1]];
    let b = memory[memory[pc+2]];
    let dest = memory[pc+3];
    //println!("adding {} and {} and writing to {}", a, b, dest);
    memory[dest] = a + b;
    true
}

fn multiply(pc: usize, memory: &mut Vec<usize>) -> bool {
    let a = memory[memory[pc+1]];
    let b = memory[memory[pc+2]];
    let dest = memory[pc+3];
    //println!("multiplying {} and {}", a, b);
    memory[dest] = a * b;
    true
}
