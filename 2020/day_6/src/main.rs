use std::collections::HashSet;
fn main() {
    let cleaned_input = include_str!("input.txt")
    .trim()
    .replace("\n\n", ";")
    .replace("\n", "");

    let part1 = cleaned_input
                .split(';')
                .fold(0, |acc, line| {
                    let mut chars = HashSet::new();
                    for ch in line.chars() {
                        chars.insert(ch);
                    }
                    acc + chars.len()
                });

    println!("{:?}", part1);
}
