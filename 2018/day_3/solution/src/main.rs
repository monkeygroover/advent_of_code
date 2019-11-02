use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input_data = include_str!("input.txt");

    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let claims = input_data.lines().map( |line| {
        let caps = re.captures(line).unwrap();
        let i = caps[1].parse::<u32>().unwrap();
        let x = caps[2].parse::<u32>().unwrap();
        let y = caps[3].parse::<u32>().unwrap();
        let w = caps[4].parse::<u32>().unwrap();
        let h = caps[5].parse::<u32>().unwrap();
        (i, x, y, w, h)
    });

    let mut fabric = vec![HashSet::new(); 1000 * 1000];
    let mut non_overlappping_claims: HashSet<u32> = HashSet::new();
    claims.clone().for_each(|(i, a, b, w, h)| {
        non_overlappping_claims.insert(i);
        for x in a..(a+w) {
            for y in b..(b+h) {
                fabric[(y*1000 + x) as usize].insert(i);
            }
        }
    });

    // Part 1
    let overlapping = fabric.iter().filter(|i| i.len() >= 2).count();
    println!("{}", overlapping);

    // Part 2
    // remove all claim ids that are in fabric cells with more than one claim
    for claim in fabric.iter() {
        if claim.len() > 1 {
            // more than one claim in this spot, remove all these claims from the non_overlappping_claims set
            for id in claim {
                non_overlappping_claims.remove(id);
            }
        }
    }
    println!("{:?}", non_overlappping_claims);
}
