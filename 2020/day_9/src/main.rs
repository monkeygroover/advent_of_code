use itertools::Itertools;

fn main() {
    let data: Vec<i64> = include_str!("input.txt")
    .trim()
    .lines()
    .map(|s| { s.parse::<i64>().unwrap()})
    .collect();

    let part1 = data.windows(26).find_map(|window|{
        let preamble = &window[..25];
        let value = window[25];
        if preamble.iter().combinations(2).all(|v| v[0] + v[1] != value) { Some(value) } else { None }
    }).unwrap();

    let part2 = (2..data.len()).find_map(|window_size| {
        data.windows(window_size).find_map(|window| {
            let window: Vec<i64> = window.to_vec();
            let sum: i64 = window.iter().sum();
            if sum == part1 { Some(window.iter().min().unwrap() + window.iter().max().unwrap()) } else { None }
        })
    }).unwrap();

    println!("{} {}", part1, part2);
}
