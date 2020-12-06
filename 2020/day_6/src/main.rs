use std::collections::HashSet;
use std::iter::FromIterator;
fn main() {
    let cleaned_input = include_str!("input.txt")
    .trim()
    .replace("\n\n", ";");

    let group_sets = cleaned_input
    .split(';')
    .map(|group| {
        group
        .lines()
        .map(|person| { HashSet::<char>::from_iter(person.chars()) })
    });

    // union
    let part1 = group_sets.clone()
    .fold(0, |acc, mut group_set| {
        let intersection_set = group_set.next()
        .map(|first_set| group_set.fold(first_set, |set1, set2| &set1 | &set2));
        acc + intersection_set.unwrap().len()
    });

    // intersection
    let part2 = group_sets
    .fold(0, |acc, mut group_set| {
        let intersection_set = group_set.next()
        .map(|first_set| group_set.fold(first_set, |set1, set2| &set1 & &set2));
        acc + intersection_set.unwrap().len()
    });

    println!("{} {}", part1, part2);
}
