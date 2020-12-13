use log::*;

use std::collections::HashMap;

fn main() {
    env_logger::init();

    let mut lines = include_str!("input.txt").trim().lines();
    let earliest_time = lines.next().map(|x| x.parse::<i64>().unwrap()).unwrap();
    let buses: HashMap<i64, i64> = lines.next()
    .map(|x| 
        x.split(",")
        .enumerate()
        .filter(|(_, x)| x.to_string() != "x")
        .map(|(i, x)| (i as i64, x.parse::<i64>().unwrap()))
        .collect::<HashMap<i64, i64>>()
    ).unwrap();

    debug!("{:?}", buses);

    let (next_bus, wait) = buses.iter()
        .map(|(_, f)| (f, f - (earliest_time % f)))
        .min_by(|(_, x), (_, y)| x.cmp(y)).unwrap();

    println!("{} * {} = {}", next_bus, wait, next_bus * wait);

    // bus ids are prime, use chinese remainder theorem
    // steal this https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
    let freqs: Vec<i64> = buses.iter().map(|(_, &freq)| freq ).collect();
    let offsets: Vec<i64> = buses.iter().map(|(&i, _)| i).collect();

    let part_2 = chinese_remainder(&offsets, &freqs).unwrap();

    println!("{}", part_2);
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
 
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
 
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
 
    let mut sum = 0;
 
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
 
    Some(sum % prod)
}
