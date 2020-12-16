use std::ops::Range;
use regex::Regex;
use itertools::Itertools;

type ValidityRule = (Range<i32>, Range<i32>);
type Ticket = Vec<i32>;

fn parse_validity(raw: &str) -> ValidityRule {
    let re = Regex::new(r"^.+: (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let caps = re.captures(raw).unwrap();

    let lo1 = caps[1].parse::<i32>().unwrap();
    let hi1 = caps[2].parse::<i32>().unwrap();

    let lo2 = caps[3].parse::<i32>().unwrap();
    let hi2 = caps[4].parse::<i32>().unwrap();

    (lo1..hi1+1, lo2..hi2+1)
}

fn is_valid(val: i32, rule: &ValidityRule) -> bool {
    rule.0.contains(&val) || rule.1.contains(&val)
}

fn main() {
    let lines = include_str!("input.txt")
                                .trim()
                                .lines();

    let rules_iter = lines.clone().take(20).map(|line| parse_validity(line));
    let invalid_for_all_rules = |val| { rules_iter.clone().all(|rule| !is_valid(val, &rule)) };

    let your_ticket: Ticket = lines.clone().skip(22).next()
                      .map(|line| line.split(",").map(|num| num.parse::<i32>().unwrap()).collect::<Ticket>() )
                      .unwrap();

    let nearby_tickets = lines.clone().skip(25)
                      .map(|line| line.split(",").map(|num| num.parse::<i32>().unwrap()).collect::<Ticket>() );

    let part_1: i32 = nearby_tickets.clone()
        .flatten()
        .fold(0, |acc, val| { if invalid_for_all_rules(val) { acc + val } else { acc } });

    println!("{:?}", part_1);

    let valid_tickets: Vec<Ticket> = nearby_tickets
        .filter(|ticket| !ticket.iter().any(|&val| invalid_for_all_rules(val) ))
        .collect();
    let rules: Vec<ValidityRule> = rules_iter.collect();

    // now need to "solve" what combination of rules (columns) match all tickets (rows)
    // rules are 0..20
    // build a table of what rules are valid for each column
    let mut valid_rules_for_column: [Vec<usize>; 20] = Default::default();

    //try each rule on each column of ticket entries
    for col in 0..20 {
        for rule in 0..20 {
            if valid_tickets.iter().all(|ticket| is_valid(ticket[col], &rules[rule] )) {
                valid_rules_for_column[col].push(rule);
            }
        }
    }
   
    for (col, rules) in valid_rules_for_column.iter().enumerate().sorted_by(|&(_, a), &(_, b)| Ord::cmp(&a.len(), &b.len()) ) { 
        println!("{:?} -> {:?}", col, rules); 
    }

    //   this prints

    // 12 -> [9]
    // 6 -> [9, 17]
    // 0 -> [9, 16, 17]
    // 5 -> [8, 9, 16, 17]
    // 16 -> [7, 8, 9, 16, 17]
    // 2 -> [7, 8, 9, 16, 17, 18]
    // 15 -> [5, 7, 8, 9, 16, 17, 18]
    // 13 -> [4, 5, 7, 8, 9, 16, 17, 18]
    // 17 -> [1, 4, 5, 7, 8, 9, 16, 17, 18]
    // 3 -> [0, 1, 4, 5, 7, 8, 9, 16, 17, 18]
    // 9 -> [0, 1, 2, 4, 5, 7, 8, 9, 16, 17, 18]
    // 1 -> [0, 1, 2, 3, 4, 5, 7, 8, 9, 16, 17, 18]
    // 10 -> [0, 1, 2, 3, 4, 5, 7, 8, 9, 16, 17, 18, 19]
    // 7 -> [0, 1, 2, 3, 4, 5, 7, 8, 9, 13, 16, 17, 18, 19]
    // 19 -> [0, 1, 2, 3, 4, 5, 7, 8, 9, 13, 14, 16, 17, 18, 19]
    // 4 -> [0, 1, 2, 3, 4, 5, 7, 8, 9, 11, 13, 14, 16, 17, 18, 19]
    // 18 -> [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 13, 14, 16, 17, 18, 19]
    // 8 -> [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14, 16, 17, 18, 19]
    // 11 -> [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19]
    // 14 -> [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]

   // lost the will to live at this point and did the rest by hand looking for the value added
   // in each step, then finding the values in "your ticket" for those columns

   println!("{}", 151u64 * 101 * 157 *  139 * 97 * 149);
}
