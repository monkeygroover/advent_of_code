use log::*;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Ins {
    Mask(String),
    Mem(i64, i64)
}

fn main() {
    env_logger::init();

    let instructions: Vec<Ins> = include_str!("input.txt")
        .trim()
        .lines()
        .map(|line| {
            if line.starts_with("mask") {
                let re = Regex::new(r"^mask = ([0|1|X]+)$").unwrap();
                let capts = re.captures(line).unwrap();
                let mask = capts[1].to_string();
                // reverse the mask to make life easier later
                let mask = mask.chars().rev().collect::<String>();

                Ins::Mask(mask)
            } else {
                let re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
                let capts = re.captures(line).unwrap();
                let addr = capts[1].parse::<i64>().unwrap();
                let val = capts[2].parse::<i64>().unwrap();
                Ins::Mem(addr, val)
            }
        })
        .collect();

    let mut memory: HashMap<i64, i64> = HashMap::new();
    let mut mask = "".to_string();
    
    for instruction in instructions {
        match instruction {
            Ins::Mask(new_mask) => {
                debug!("Set mask {}", new_mask);
                mask = new_mask;
            }
            Ins::Mem(addr, val) => {
                let mut address_iter = mask.chars().enumerate();
                let (_, first_mask) = address_iter.next().unwrap();
                let acc: Vec<i64> = match first_mask {
                    '0' => vec![addr & 1],
                    '1' => vec![1],
                    'X' => vec![0,1],
                    n => panic!("{}", n)
                };

                let addresses = address_iter.fold(acc,
                    |acc, (n, ch)| {
                        trace!("{}, {}, {:?}", n, ch, acc);
                        match ch {
                            '0' => {
                                acc.iter().map(|add| add + (addr & (1 << n)) ).collect()
                            },
                            '1' => {
                                acc.iter().map(|add| add + (1 << n)).collect()
                            },
                            'X' => {
                                // duplicate all the values so far then add 1 and 0
                                let mut addresses: Vec<i64> = vec![];
                                for add in acc {
                                    addresses.push(add);
                                    addresses.push(add + (1 << n));
                                };
                                addresses
                            },
                            _ => unreachable!()
                        }
                    }
                );
            
                debug!("addresses {:?}", addresses);
                for addr in addresses {
                    memory.insert(addr,val);
                }
            }
        }
    }

    println!("part_2 {}", memory.values().sum::<i64>());
}
