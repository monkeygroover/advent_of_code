use std::collections::HashSet;
use std::iter::FromIterator;
fn main() {
    let cleaned_input = include_str!("input.txt")
    .trim()
    .replace("\n\n", ";");

    let part1 = cleaned_input
    .replace("\n", "")
    .split(';')
    .fold(0, |acc, line| { acc + HashSet::<char>::from_iter(line.chars()).len() });

    let part2 = cleaned_input
    .split(';')
    .fold(0, |acc, group| {
        let mut answer_sets = group
        .lines()
        .map(|person| { HashSet::<char>::from_iter(person.chars()) });

        let intersection_set = answer_sets.next()
        .map(|first_set| answer_sets.fold(first_set, |set1, set2| &set1 & &set2));
        acc + intersection_set.unwrap().len()
    });

    println!("{} {}", part1, part2);
}
