use log::*;
use regex::Regex;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Ship {
    x: i32,
    y: i32,
    wp_x: i32, // relative to ship
    wp_y: i32
}

#[derive(Debug)]
enum Rotate {
    Ninety,
    OneEighty,
    TwoSeventy
}

impl Ship {
    fn new() -> Ship {
        Ship{x: 0, y: 0, wp_x: 10, wp_y: 1}
    }

    fn north(&self, n: i32) -> Ship {
        debug!("north {}", n);
        Ship{wp_y: self.wp_y + n, ..*self}
    }

    fn south(&self, s: i32) -> Ship {
        debug!("south {}", s);
        Ship{wp_y: self.wp_y - s, ..*self}
    }

    fn east(&self, e: i32) -> Ship {
        debug!("east {}", e);
        Ship{wp_x: self.wp_x + e, ..*self}
    }

    fn west(&self, w: i32) -> Ship {
        debug!("west {}", w);
        Ship{wp_x: self.wp_x - w, ..*self}
    }

    fn rotate(&self, r: Rotate) -> Ship {
        debug!("rotate {:?}", r);

        let (rot_x, rot_y) = match r {
            Rotate::Ninety => (0 - self.wp_y , self.wp_x),
            Rotate::OneEighty => (0 - self.wp_x, 0 - self.wp_y ),
            Rotate::TwoSeventy => (self.wp_y , 0 - self.wp_x)
        };

        debug!("dx {}, dy {} : rot_x {}, rot_y {}", self.wp_x, self.wp_y, rot_x, rot_y);

        Ship{wp_x: rot_x, 
             wp_y: rot_y,
             ..*self}
    }

    fn forward(&self, n: i32) -> Ship {
        debug!("forward {}", n);
        (0..n).fold(*self, |ship, _| Ship{x: ship.x + ship.wp_x, y: ship.y + ship.wp_y, ..ship})
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
    Left(i32),
    Right(i32),
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
            "L" => Command::Left(val),
            "R" => Command::Right(val),
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
            Command::Left(l)  => {
                match l {
                    90 => ship.rotate(Rotate::Ninety),
                    180 => ship.rotate(Rotate::OneEighty),
                    270 => ship.rotate(Rotate::TwoSeventy),
                    l => panic!("bad rotation l {}", l)
                }
            },
            Command::Right(r) => {
                match r {
                    90 => ship.rotate(Rotate::TwoSeventy),
                    180 => ship.rotate(Rotate::OneEighty),
                    270 => ship.rotate(Rotate::Ninety),
                    r => panic!("bad rotation r {}", r)
                }
            },
            Command::Forward(f) => ship.forward(f)
        }
    });

    debug!("{:?}", ship);

    println!("{}", ship.manhattan());
}
