use log::*;

#[derive(Debug)]
enum ShuffleType {
    DealNew,
    Cut(i32),
    DealIncrement(i32)
}

impl ShuffleType {
    fn from_command(line: &str) -> ShuffleType {
        if line.starts_with("deal into new stack") {
            return ShuffleType::DealNew
        }
        if line.starts_with("cut ") {
            let slice = &line[4..];
            return ShuffleType::Cut(slice.parse::<i32>().unwrap())
        }

        if line.starts_with("deal with increment ") {
            let slice = &line[20..];
            return ShuffleType::DealIncrement(slice.parse::<i32>().unwrap())
        }

        panic!("bad command");
    }
}


struct Deck {
    cards: Vec<i32>
}

impl Deck {
    fn new() -> Deck {
        Deck{cards: (0..10007).collect()}
    }

    fn deal_new(&mut self) {
        self.cards = self.cards.iter().rev().cloned().collect::<Vec<i32>>();
    }

    fn cut(&mut self, n: i32) {
        if n >= 0 {
            let p1 = self.cards.iter().take(n as usize);
            let mut p2 = self.cards.iter().skip(n as usize).cloned().collect::<Vec<i32>>();

            p2.extend(p1);
            self.cards = p2;
        } else {
            let skip = self.cards.len() - n.abs() as usize;
            let mut p1 = self.cards.iter().skip(skip).cloned().collect::<Vec<i32>>();
            let p2 = self.cards.iter().take(skip);

            p1.extend(p2);
            self.cards = p1;
        }
    }

    fn deal_inc(&mut self, n: i32) {
        let len = self.cards.len();
        let mut new_vec = vec![0;len];

        let mut insert_pos: usize = 0;

        for card in self.cards.clone() {
            new_vec[insert_pos] = card;
            insert_pos += n as usize;
            insert_pos %= len;
        }

        self.cards = new_vec;
    }
}

fn main() {
    env_logger::init();

    let shuffles = include_str!("input.txt")
    .trim()
    .lines()
    .map(|c| ShuffleType::from_command(c));

    let mut deck = Deck::new();

    shuffles.for_each(|command| {
        match command {
            ShuffleType::DealNew => deck.deal_new(),
            ShuffleType::Cut(n) => deck.cut(n),
            ShuffleType::DealIncrement(n) => deck.deal_inc(n)
        }
    } );

    println!("part1: {}", deck.cards.iter().position(|x| *x == 2019).unwrap())

}
