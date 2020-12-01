use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt").trim().lines().map(|s| s.parse::<u32>().unwrap());

    let part1 = input.clone().combinations(2)
                             .find(|v| v[0] + v[1] == 2020)
                              .map(|v| v[0] * v[1]);

    let part2 = input.combinations(3)
                     .find(|v| v[0] + v[1] + v[2] == 2020)
                      .map(|v| v[0] * v[1] * v[2]);

    println!("{}, {}", part1.unwrap(), part2.unwrap());
}
