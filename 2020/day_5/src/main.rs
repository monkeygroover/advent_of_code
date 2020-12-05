use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt").trim().lines().map(|s| s.to_string());

    let seat_ids = input.map(|line| {
        let binary_str = line.replace("F", "0").replace("B", "1").replace("L", "0").replace("R", "1");
        let row = isize::from_str_radix(&binary_str[..7], 2).unwrap();
        let column = isize::from_str_radix(&binary_str[7..10], 2).unwrap();
        row * 8 + column
    });

    let part1 = seat_ids.clone().max().unwrap();
    let (seat_before_gap, _) = seat_ids.sorted().tuple_windows().find(|(a, b)| *b != a + 1).unwrap();
    let part2 = seat_before_gap + 1;
    
    println!("{} {}", part1, part2);
}
