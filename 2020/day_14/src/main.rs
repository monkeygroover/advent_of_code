use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Ins {
    Mask((i64, i64)),
    Mem(usize, i64)
}

fn main() {
    let instructions: Vec<Ins> = include_str!("input.txt")
        .trim()
        .lines()
        .map(|line| {
            if line.starts_with("mask") {
                let re = Regex::new(r"^mask = ([0|1|X]+)$").unwrap();
                let capts = re.captures(line).unwrap();
                let mask = capts[1].to_string();

                let ones_mask_str = mask.replace("X", "0");
                let ones_mask = i64::from_str_radix(&ones_mask_str, 2).unwrap();

                let zeros_mask_str = mask.replace("X", "1");
                let zeros_mask = i64::from_str_radix(&zeros_mask_str, 2).unwrap();
                Ins::Mask((ones_mask, zeros_mask))
            } else {
                let re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
                let capts = re.captures(line).unwrap();
                let addr = capts[1].parse::<usize>().unwrap();
                let val = capts[2].parse::<i64>().unwrap();
                Ins::Mem(addr, val)
            }
        })
        .collect();

    let mut memory: Vec<i64> = vec![0;70000];
    let mut mask = (0,0);
    
    for instruction in instructions {
        match instruction {
            Ins::Mask(new_mask) => {
                mask = new_mask;
            }
            Ins::Mem(addr, val) => {
                let (ones, zeros) = mask;
                memory[addr] = (val | ones) & zeros;
            }
        }
    }

    println!("part_1 {}", memory.into_iter().sum::<i64>());
}
