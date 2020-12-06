use std::collections::HashSet;
fn main() {
    let cleaned_input = include_str!("input.txt")
    .trim()
    .replace("\n\n", ";");

    let part1 = cleaned_input
                .replace("\n", "")
                .split(';')
                .fold(0, |acc, line| {
                    let mut chars = HashSet::new();
                    for ch in line.chars() {
                        chars.insert(ch);
                    }
                    acc + chars.len()
                });

    let part2 = cleaned_input
    .split(';')
    .fold(0, |acc, group| {
        let mut answers: Vec<HashSet<char>> = vec![];
        for person in group.lines() {
            let mut chars = HashSet::new();
            for ch in person.chars() {
                chars.insert(ch);
            }
            answers.push(chars)
        }
        let intersection_set = answers.iter().fold(answers[0].clone(), |acc, a| acc.intersection(&a).cloned().collect());
        acc + intersection_set.len()
    });


    println!("{} {}", part1, part2);
}
