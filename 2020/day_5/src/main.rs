use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt")
    .trim()
    .lines();

    let row_list: Vec<i32> = (0..128).collect();
    let column_list: Vec<i32> = (0..8).collect();

    let seat_ids = input.map(|line| {
        let first_7 = line.chars().take(7);
        let last_3 = line.chars().skip(7).take(3);

        let column = first_7.fold(row_list.clone(),
            |mut acc, half| match half {
                'F' => {
                    let split_point = acc.len() / 2;
                    acc.truncate(split_point);
                    acc
                },
                'B' => {
                    let split_point = acc.len() / 2;
                    let split = acc.split_off(split_point);
                    split
                },
                _ => panic!("Bad value")
            }
        );

        let row = last_3.fold(column_list.clone(),
            |mut acc, half| match half {
                'L' => {
                let split_point = acc.len() / 2;
                    acc.truncate(split_point);
                    acc
                },
                'R' => {
                    let split_point = acc.len() / 2;
                    let split = acc.split_off(split_point);
                    split
                },
                _ => panic!("Bad value")
            }
        );

        column[0] * 8 + row[0]
    });

    let part1 = seat_ids.clone().max().unwrap();

    let (seat_before_gap, _) = seat_ids.sorted().tuple_windows().find(|(a, b)| *b != a + 1).unwrap();
    let part2 = seat_before_gap + 1;
    
    println!("{} {}", part1, part2);
}
