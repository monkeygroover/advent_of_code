use std::collections::HashSet;
fn main() {
    let cleaned_input = include_str!("input.txt")
    .trim()
    .replace("\n\n", ";")
    .replace("\n", "");

    let data: Vec<HashSet<char>> = cleaned_input
                .split(';')
                .map(|line| {
                    let mut chars = HashSet::new();
                    for ch in line.chars() {
                        chars.insert(ch);
                    }
                    chars
                })
                .collect();

    let part1 = data.iter().fold(0, |acc, set| acc + set.len());

    println!("{:?}", part1);
}
