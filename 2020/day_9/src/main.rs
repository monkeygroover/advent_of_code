use itertools::Itertools;

fn main() {

    let data: Vec<i64> = include_str!("input.txt")
    .trim()
    .lines()
    .map(|s| { s.parse::<i64>().unwrap()})
    .collect();

    for window in data.windows(26) {
        let values = &window[..25];
        let to_check = window.last().unwrap();

        if values.iter().combinations(2).all(|v| v[0] + v[1] != *to_check ) {
            println!("{:?}", to_check);
            break;
        }
    }

    for window_size in 2..data.len() {
        for window in data.windows(window_size) {
            let window: Vec<i64> = window.to_vec();
            let sum: i64 = window.iter().sum();
            if sum == 530627549 {
                let part2 = window.iter().min().unwrap() + window.iter().max().unwrap();
                println!("{}", part2);
            }
        }
    }

}
