use std::collections::VecDeque;

fn main() {
    let input_lines = include_str!("input.txt").trim().lines();

    let mut player_1_stack: VecDeque<u32> = input_lines.clone().skip(1).take(25).map(|x| x.parse::<u32>().unwrap()).collect();
    let mut player_2_stack: VecDeque<u32> = input_lines.skip(28).take(25).map(|x| x.parse::<u32>().unwrap()).collect();

    let result = loop {
        if let Some(result) = play_round(&mut player_1_stack, &mut player_2_stack) {
            break result;
        }
    };

    let part_1 = result.iter().rev().enumerate().fold(0, |acc, (n, x)| acc + (n as u32 + 1) * x);

    println!("{:?}", part_1);
}

fn play_round(p1: &mut VecDeque<u32>, p2: &mut VecDeque<u32>) -> Option<VecDeque<u32>> {
    let top_p1 = p1.pop_front().unwrap();
    let top_p2 = p2.pop_front().unwrap();

    if top_p1 > top_p2 {
        p1.push_back(top_p1);
        p1.push_back(top_p2);
    } else {
        p2.push_back(top_p2);
        p2.push_back(top_p1);
    }

    if p1.len() == 0 { return Some(p2.clone()); }
    if p2.len() == 0 { return Some(p1.clone()); }
    None
}
