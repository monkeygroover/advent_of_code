use std::ops::Range;
use regex::Regex;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct FieldValidity {
    name: String,
    ranges: Vec<Range<i32>>
}

impl From<&str> for FieldValidity {
    fn from(raw: &str) -> Self {
        let re = Regex::new(r"^(.+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        let caps = re.captures(raw).unwrap();
        let name = caps[1].to_string();

        let mut ranges: Vec<Range<i32>> = vec![];
        let lo1 = caps[2].parse::<i32>().unwrap();
        let hi1 = caps[3].parse::<i32>().unwrap();
        ranges.push(lo1..hi1+1);

        let lo2 = caps[4].parse::<i32>().unwrap();
        let hi2 = caps[5].parse::<i32>().unwrap();
        ranges.push(lo2..hi2+1);

        FieldValidity{name: name, ranges: ranges}
    }
}

impl FieldValidity {
    fn is_valid(self, val: i32) -> bool {
        self.ranges.iter().any(|range| range.contains(&val))
    }
}

fn main() {
    let lines = include_str!("input.txt")
                                .trim()
                                .lines();

    let validities_iter = lines.clone().take(20).map(|line| FieldValidity::from(line));
    let all_invalid = |val| { validities_iter.clone().all(|validity| !validity.is_valid(val)) };

    let _your_ticket: Vec<i32> = lines.clone().skip(22).next()
                      .map(|line| line.split(",").map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>() )
                      .unwrap();

    let nearby_tickets = lines.clone().skip(25)
                      .map(|line| line.split(",").map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>() );

    let nearby_tickets_flat: Vec<i32> = nearby_tickets.flatten().collect();

    let part_1: i32 = nearby_tickets_flat.iter()
        .fold(0, |acc, &val| { if all_invalid(val) { acc + val } else { acc } });

    println!("{}", part_1);
}
