use itertools::Itertools;

fn main() {
    let seat_ids = include_str!("input.txt").trim().lines()
        .map(|s| {
            let binary_str = s.to_string().replace("F", "0").replace("B", "1").replace("L", "0").replace("R", "1");
            isize::from_str_radix(&binary_str, 2).unwrap() });

    let part1 = seat_ids.clone().max().unwrap();
    let part2 = seat_ids.sorted().tuple_windows().find(|(a, b)| *b != a + 1).map(|(seat_before_gap, _)| seat_before_gap + 1 ).unwrap();
    
    println!("{} {}", part1, part2);
}
