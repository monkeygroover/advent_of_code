fn main() {
    let initial_data: Vec<i64> = include_str!("input.txt")
    .trim()
    .chars()
    .map(|c| c.to_string().parse::<i64>().unwrap())
    .collect();


    let data = (0..100).fold(initial_data, |data, _acc| {
        phase(data)
    });

    println!("part1: {:?}", data.iter().take(8).map(|d| d.to_string()).collect::<Vec<String>>());
}


fn phase(input: Vec<i64>) -> Vec<i64> {
    let pattern = |base: Vec<i64>, repeat: usize| {
        let mut count = 0;
        let mut repeat_count = 0;
        std::iter::from_fn(move || {
            repeat_count += 1;
            if repeat_count == repeat {
                count += 1;
                if count == base.len() {count = 0}
                repeat_count = 0;
            }
            Some(base[count])
        })
    };

    (1..=input.len()).fold(
        vec![], |mut acc : Vec<i64>, i| {
            let pat = pattern(vec![0,1,0,-1], i);
            let new_digit = input.clone().iter().zip(pat).collect::<Vec<(&i64, i64)>>()
            .iter()
            .fold(0, |acc, (i, val)| {
                acc + (**i * val)
            }
            );
            acc.push(new_digit.abs() % 10);
            acc
        })
}
