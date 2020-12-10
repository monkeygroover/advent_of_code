use log::*;

#[derive(Debug, Copy, Clone)]
struct Acc {
    combs: i64,
    run: isize
}

fn main() {
    env_logger::init();

    let mut input: Vec<i32> = include_str!("input.txt")
    .trim()
    .lines()
    .map(|s| { s.parse::<i32>().unwrap()} )
    .collect();

    input.sort();

    input.insert(0,0);
    input.push(input.iter().max().map(|x| x + 3).unwrap());
    let input = input;

    debug!("{:?}", input);

    let differences = input.windows(2)
    .map(|jolt_pair| jolt_pair[1] - jolt_pair[0]);

    let diffs_of_one = differences.clone().filter(|&d| d == 1 ).count();
    let diffs_of_three = differences.clone().filter(|&d| d == 3 ).count();
    let part1 = diffs_of_one * diffs_of_three;

    println!("part_1 {} * {} = {}", diffs_of_one, diffs_of_three, part1);

    let diffs_of_two = differences.clone().filter(|&d| d == 2 ).count();
    debug!("diffs_of_two {}", diffs_of_two);

    // part 2
    // once we hit a difference of 3, there is only going to be one way to plug that
    // so if we scan looking for gaps of 3 then each set of gaps inbetween becomes a smaller problem to solve
    
    // to solve the sub problem.. first note there are NO plug diffs of two in the input, so if it isn't 3, it's 1
    // sub problem how many combinations of ways of plugging are there in a run of 1's ... 
    // it's just (calculated by hand) the sequence 1, 2, 4, 7 (there are no runs of more than 5 plugs (i.e. 4 gaps))

    let part_2: Acc = differences.fold(
        Acc{combs: 1, run: 0},
        |acc, diff| {
            if diff == 3 {
                // only one way to plug a gap of 3 so we can combine the combinations in the last run and reset run
                Acc{combs: acc.combs * run_combs(acc.run), run: 0}
            } else {
                Acc{combs: acc.combs, run: acc.run + 1}
            }
        });

    println!("{:?}", part_2.combs);
}

fn run_combs(run: isize) -> i64 {
    match run {
        0 => 1,
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 7,
        x => panic!("run {} too long", x)
    }
}
