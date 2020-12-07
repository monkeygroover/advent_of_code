use std::collections::HashMap;
use regex::Regex;

fn main() {
    let bags: HashMap<String, Vec<(u32, String)> > = include_str!("input.txt")
    .trim()
    .lines()
    .map(|line| {
        let re = Regex::new(r"^(.*) bags contain (.*).$").unwrap();
        let captures = re.captures(line).unwrap();
        let bag_name = captures[1].to_string();

        let contains: Vec<(u32, String)> = captures[2].to_string().split(", ").flat_map(|inner_bag| {
            let inner_bag = inner_bag.to_string();
            match inner_bag.as_str() {
                "no other bags" => vec![],
                inner_bag => {
                    let re = Regex::new(r"(\d+) (.*) bags?").unwrap();
                    let captures = re.captures(inner_bag).unwrap();
                    let count = captures[1].parse::<u32>().unwrap();
                    let bag = captures[2].to_string();
                    vec![(count, bag.to_string().trim().to_string())]
                }
            }
        }).collect();
        
        (bag_name, contains)
    }).collect();

    let bags_that_can_contain_gold = 
        bags.iter().filter(|&(bag_name, _bag_contents)| {
                bag_contains_bag(&bags, String::from(bag_name), "shiny gold".to_string() )
            }
        );
        
    let bags_inside_gold = count_bags(&bags, "shiny gold".to_string());
    
    println!("{}, {}", bags_that_can_contain_gold.count() - 1, bags_inside_gold - 1);
}

fn bag_contains_bag(bags: &HashMap<String, Vec<(u32, String)>>, bag_name: String, bag_to_check: String) -> bool {
    if bag_name == bag_to_check {
        true
    } else {
        let contained_bags = bags.get(&bag_name).unwrap();

        if contained_bags.len() == 0 {
            false
        } else {
             contained_bags.iter().any(|(_, i)| bag_contains_bag(&bags, i.clone(), bag_to_check.clone()))
        }
    }
}

fn count_bags(bags: &HashMap<String, Vec<(u32, String)>>, bag_name: String) -> u32 {
    let inner_bags = bags.get(&bag_name).unwrap();
    1 + inner_bags.iter().fold(0, |acc, (count, name)| acc + count * count_bags(bags, String::from(name)))
}
