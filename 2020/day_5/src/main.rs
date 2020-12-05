use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
enum Split {
    Lower,
    Upper
}

fn main() {
    let input = include_str!("input.txt")
    .trim()
    .lines();

    let row_list: Vec<i32> = (0..128).collect();
    let column_list: Vec<i32> = (0..8).collect();

    let seat_ids = input.map(|line| {
        let first_7: Vec<Split> = line.chars().take(7).map(|ch| match ch { 'F' => Split::Lower, 'B' => Split::Upper, _ => panic!("bad")}).collect();
        let row = bin_search(row_list.clone(), first_7);

        let last_3: Vec<Split> = line.chars().skip(7).take(3).map(|ch| match ch { 'L' => Split::Lower, 'R' => Split::Upper, _ => panic!("bad")}).collect();
        let column = bin_search(column_list.clone(), last_3);

        row * 8 + column
    });

    let part1 = seat_ids.clone().max().unwrap();

    let (seat_before_gap, _) = seat_ids.sorted().tuple_windows().find(|(a, b)| *b != a + 1).unwrap();
    let part2 = seat_before_gap + 1;
    
    println!("{} {}", part1, part2);
}

fn bin_search(search_list: Vec<i32>, split_sequence: Vec<Split>) -> i32 {
    let list = split_sequence.iter().fold(
        search_list,
        |mut acc, half| match half {
            Split::Lower => {
                let split_point = acc.len() / 2;
                acc.truncate(split_point);
                acc
            },
            Split::Upper => {
                let split_point = acc.len() / 2;
                let split = acc.split_off(split_point);
                split
            }
        }
    );

    list[0]
}
