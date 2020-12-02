use regex::Regex;
// use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Password {
    ch: char,
    lo: u32,
    hi: u32,
    pass: String
}

impl Password {
    fn parse(raw: &str) -> Password {
        let re = Regex::new(r"([\d]+)-([\d]+) ([a-z]): ([a-z]+)").unwrap();
        let caps = re.captures(raw).unwrap();
        let lo = caps[1].parse::<u32>().unwrap();
        let hi = caps[2].parse::<u32>().unwrap();
        let ch = caps[3].parse::<char>().unwrap();
        let pass = caps[4].to_string();

        Password{ch: ch, lo: lo, hi: hi, pass: pass}
    }

    fn is_valid(&self) -> bool {
        let chars_in_pass = self.pass.chars().filter(|ch| *ch == self.ch).count() as u32;
        chars_in_pass >= self.lo && chars_in_pass <= self.hi
    }
}


fn main() {
    let passwords = include_str!("input.txt")
                                .trim()
                                .lines()
                                .map(|line| Password::parse(line));

    let valid_count = passwords.filter(|pw| pw.is_valid()).count();

    println!("{:?}", valid_count);
}
