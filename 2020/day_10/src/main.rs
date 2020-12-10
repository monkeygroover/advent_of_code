#[derive(Debug, Copy, Clone)]
struct Acc {
    combs: i64,
    run: i64
}

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

    let differences = input.windows(2)
    .map(|jolt_pair| jolt_pair[1] - jolt_pair[0]);

    let diffs_of_one = differences.clone().filter(|&d| d == 1 ).count();
    let diffs_of_three = differences.clone().filter(|&d| d == 3 ).count();
    let part1 = diffs_of_one * diffs_of_three;

    println!("part_1 {} * {} = {}", diffs_of_one, diffs_of_three, part1);

    let diffs_of_two = differences.clone().filter(|&d| d == 2 ).count();
    println!("diffs_of_two {}", diffs_of_two);

    for i in 0..10 {
        println!("triangles {} {}", i, triangle(i) + 1);
    }
    // part 2
    // once we hit a difference of 3, there is only going to be one way to solve that
    // so if we scan looking for gaps of 3 then each gap inbetween becomes a smaller problen to solve
    // to solve the sub problem..

    // note there are NO diffs of two in the input, so if it isn't 3, it's 1
    // sub problem how many combinations are there in a run of 1's ... it's just
    // the sequence 1, 2, 4, 7, 11... (triangular numbers + 1) 

    let part_2: Acc = differences.fold(
        Acc{combs: 1, run: 0},
        |acc, diff| {
            if diff == 3 {
                // only one way to plug a gap of 3 so we can total the combinations in the last run and reset
                Acc{combs: acc.combs * (triangle(acc.run-1) + 1), run: 0}
            } else {
                Acc{combs: acc.combs, run: acc.run + 1}
            }
        });

    println!("{:?}", part_2);
}

fn triangle(n: i64) -> i64 {
    (n * (n + 1) / 2) as i64
}
