use log::*;
use regex::Regex;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Ship {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32
}

impl Ship {
    fn new() -> Ship {
        Ship{x: 0, y: 0, dx: 1, dy: 0}
    }

    fn north(&self, n: i32) -> Ship {
        debug!("north {}", n);
        Ship{y: self.y + n, ..*self}
    }

    fn south(&self, s: i32) -> Ship {
        debug!("south {}", s);
        Ship{y: self.y - s, ..*self}
    }

    fn east(&self, e: i32) -> Ship {
        debug!("east {}", e);
        Ship{x: self.x + e, ..*self}
    }

    fn west(&self, w: i32) -> Ship {
        debug!("west {}", w);
        Ship{x: self.x - w, ..*self}
    }

    fn left(&self, n: usize) -> Ship {
        debug!("left {}", n);
        (0..n).fold(*self, |ship, _| ship.do_left())
    }

    fn do_left(&self) -> Ship {
        let (new_dx, new_dy) = match (self.dx, self.dy) {
            (1,0) => (0,1),
            (0,1) => (-1, 0),
            (-1, 0) => (0, -1),
            (0, -1) => (1, 0),
            _ => unreachable!()
        };
        Ship{dx: new_dx, dy: new_dy, ..*self}
    }

    fn right(&self, n: usize) -> Ship {
        debug!("right {}", n);
        (0..n).fold(*self, |ship, _| ship.do_right())
    }

    fn do_right(&self) -> Ship {
        let (new_dx, new_dy) = match (self.dx, self.dy) {
            (1,0) => (0,-1),
            (0,-1) => (-1, 0),
            (-1, 0) => (0, 1),
            (0, 1) => (1, 0),
            _ => unreachable!()
        };
        Ship{dx: new_dx, dy: new_dy, ..*self}
    }

    fn forward(&self, f: i32) -> Ship {
        debug!("forward {}", f);
        Ship{x: self.x + self.dx * f , y: self.y + self.dy * f, ..*self}
    }

    fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

enum Command {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(usize),
    Right(usize),
    Forward(i32)
}

fn main() {
    env_logger::init();

    let re = Regex::new(r"(N|S|E|W|L|R|F)(\d+)").unwrap();

    let commands = include_str!("input.txt")
    .trim().lines()
    .map(|line| {
        let caps = re.captures(line).unwrap();
        let command = caps[1].parse::<String>().unwrap().to_owned();
        let val = caps[2].parse::<i32>().unwrap();
        match command.as_ref() {
            "N" => Command::North(val),
            "S" => Command::South(val),
            "E" => Command::East(val),
            "W" => Command::West(val),
            "L" => Command::Left((val/90) as usize),
            "R" => Command::Right((val/90) as usize),
            "F" => Command::Forward(val),
            _   => panic!("bad direction")
        }
    } );

    let ship = commands.fold(Ship::new(), |ship, command| {
        debug!("{:?}", ship);
        match command {
            Command::North(n) => ship.north(n),
            Command::South(s) => ship.south(s),
            Command::East(e)  => ship.east(e),
            Command::West(w)  => ship.west(w),
            Command::Left(l)  => ship.left(l),
            Command::Right(r) => ship.right(r),
            Command::Forward(f) => ship.forward(f)
        }
    });

    debug!("{:?}", ship);

    println!("{}", ship.manhattan());
}
