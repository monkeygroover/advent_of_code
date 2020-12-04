use std::collections::HashMap;
use regex::Regex;

fn main() {
    let cleaned_input = include_str!("input.txt")
    .trim()
    .replace("\n\n", ";");

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

    let valid_count_1 = data.iter().filter(|record| is_valid_1(record)).count();
    let valid_count_2 = data.iter().filter(|record| is_valid_2(record)).count();

    println!("{} {}", valid_count_1, valid_count_2);

}

fn is_valid_1(record: &HashMap<&str, &str>) -> bool {
    record.contains_key("byr") &&
    record.contains_key("iyr") &&
    record.contains_key("eyr") &&
    record.contains_key("hgt") &&
    record.contains_key("hcl") &&
    record.contains_key("ecl") && 
    record.contains_key("pid")
}

fn is_valid_2(record: &HashMap<&str, &str>) -> bool {
    record.contains_key("byr") && matches_regex(r"^\d{4}$", record["byr"]) && in_range(1920, 2002, record["byr"]) &&
    record.contains_key("iyr") && matches_regex(r"^\d{4}$", record["iyr"]) && in_range(2010, 2020, record["iyr"]) &&
    record.contains_key("eyr") && matches_regex(r"^\d{4}$", record["eyr"]) && in_range(2020, 2030, record["eyr"]) &&
    record.contains_key("hgt") && height_valid(record["hgt"]) &&
    record.contains_key("hcl") && matches_regex(r"^#[0-9a-f]{6}$", record["hcl"]) && 
    record.contains_key("ecl") && matches_regex(r"^(amb|blu|brn|gry|grn|hzl|oth)$", record["ecl"]) &&
    record.contains_key("pid") && matches_regex(r"^\d{9}$", record["pid"])
}

fn matches_regex(re: &str, test: &str) -> bool {
    let re = Regex::new(re).unwrap();
    re.is_match(test)
}

fn in_range(lo: u32, hi: u32, test: &str) -> bool {
    let value = test.parse::<u32>().unwrap();
    value >= lo && value <= hi
}

fn height_valid(test: &str) -> bool {
    let re = Regex::new(r"(\d+)(in|cm)").unwrap();
    if re.is_match(test) {
        let caps = re.captures(test).unwrap();
            if caps[2] == "cm".to_string() { return in_range(150, 193, &caps[1]) }
            else { return in_range(59, 76, &caps[1]) }
    }
        
    false
}
