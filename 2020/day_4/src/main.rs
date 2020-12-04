use std::collections::HashMap;

fn main() {
    let cleaned_input = include_str!("input.txt")
    .trim()
    .replace("\n\n", ";")
    .replace("\n", " ");

    let data: Vec<HashMap<&str, &str>> = cleaned_input
                .split(';')
                .map(|line|
                    line.split_whitespace().fold(HashMap::new(),
                        |mut acc, kvpart| {
                            let split: Vec<&str> = kvpart.split(':').collect();
                            acc.insert(split[0], split[1]);
                            acc
                        })
                )
                .collect();
    // println!("{:?}", data);

    let valid_count = data.iter()
    .filter(|record| is_valid(record))
    .count();

    println!("{}", valid_count);

}

fn is_valid(record: &HashMap<&str, &str>) -> bool {
    record.contains_key("byr") &&
    record.contains_key("iyr") &&
    record.contains_key("eyr") &&
    record.contains_key("hgt") &&
    record.contains_key("hcl") &&
    record.contains_key("ecl") &&
    record.contains_key("pid")
}
