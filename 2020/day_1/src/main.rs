use itertools::Itertools;

fn main() {
    let input_iter = include_str!("input.txt")
                                 .trim()
                                 .lines()
                                 .map(|s| s.parse::<u32>().unwrap());

    for v in input_iter.clone().combinations(2) {
        if v[0] + v[1] == 2020 {
            println!("{}", v[0] * v[1])
        }
    }

    for v in input_iter.combinations(3) {
        if v[0] + v[1] + v[2] == 2020 {
            println!("{}", v[0] * v[1] * v[2])
        }
    }
}
