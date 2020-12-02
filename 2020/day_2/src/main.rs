use regex::Regex;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Password {
    ch: char,
    lo: usize,
    hi: usize,
    pass: String
}

impl From<&str> for Password {
    fn from(raw: &str) -> Self {
        let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
        let caps = re.captures(raw).unwrap();
        let lo = caps[1].parse::<usize>().unwrap();
        let hi = caps[2].parse::<usize>().unwrap();
        let ch = caps[3].parse::<char>().unwrap();
        let pass = caps[4].to_string();

        Password{ch: ch, lo: lo, hi: hi, pass: pass}
    }
}

impl Password {
    fn is_valid(&self) -> bool {
        let chars_in_pass = self.pass.chars().filter(|ch| *ch == self.ch).count();
        chars_in_pass >= self.lo && chars_in_pass <= self.hi
    }

    fn is_valid2(&self) -> bool {
        let check_first = self.pass.chars().nth(self.lo - 1).unwrap() == self.ch;
        let check_second = self.pass.chars().nth(self.hi - 1).unwrap() == self.ch;
        (check_first || check_second) && !(check_first && check_second)
    }
}

fn main() {
    let passwords = include_str!("input.txt")
                                .trim()
                                .lines()
                                .map(|line| Password::from(line));

    let valid_count = passwords.clone().filter(|pw| pw.is_valid()).count();
    let valid2_count = passwords.filter(|pw| pw.is_valid2()).count();

    println!("{}, {}", valid_count, valid2_count);
}
