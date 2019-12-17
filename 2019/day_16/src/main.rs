fn main() {
    let initial_data: Vec<i64> = include_str!("input.txt")
    .trim()
    .chars()
    .map(|c| c.to_string().parse::<i64>().unwrap())
    .collect();

    let offset = initial_data.clone().iter().enumerate().take(7).fold(0, |acc, (i, digit)| {
        acc + digit * 10i64.pow(6 - i as u32)
    }) as usize;

    let repeated_data = initial_data.repeat(10000).iter().cloned().skip(offset).collect::<Vec<i64>>();

    let data2 = (0..100).fold(repeated_data, |data, _acc| {
        phase(data)
    });

    let output2 = data2.iter().enumerate().take(8).fold(0, |acc, (i, digit)| {
        acc + digit * 10i64.pow(7 - i as u32)
    });

    println!("part2: {}", output2);
}

fn phase(input: Vec<i64>) -> Vec<i64> {
    let len = input.len();
    let mut new_array = vec![0;len];

    new_array[len - 1] = input[len - 1];

    for (i, val) in input.iter().rev().enumerate().skip(1) {
        new_array[len - i - 1] = (new_array[len - i] + val) % 10;
    }

    new_array
}
