fn main() {
    let input: Vec<i32> = include_str!("input.txt")
    .trim()
    .lines()
    .map(|s| { s.parse::<i32>().unwrap()} )
    .collect();

    println!("{:?}", input)
}
