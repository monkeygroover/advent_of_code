fn main() {
    let mut input: Vec<i32> = include_str!("input.txt")
    .trim()
    .lines()
    .map(|s| { s.parse::<i32>().unwrap()} )
    .collect();

    input.sort();

    input.insert(0,0);
    input.push(input.iter().max().map(|x| x + 3).unwrap());
    let input = input;

    println!("{:?}", input);

    let (diffs_of_one, diffs_of_three) = 
        input.windows(2)
        .fold( (0,0), |(o, t), j_pair| {
            let diff = j_pair[1] -j_pair[0];
            match diff {
                1 => (o+1, t),
                3 => (o, t+1),
                _ => unreachable!()
            }
        });

    let part1 = diffs_of_one * diffs_of_three;

    println!("{}*{}={}", diffs_of_one, diffs_of_three, part1)
}
