use log::*;

use num_rational::Rational;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Coord {
    x: isize,
    y: isize
}

impl Coord {
    fn new(x: isize, y: isize) -> Coord {
        Coord{x: x, y: y}
    }

    fn vector_to(self, other: Coord) -> Vector {
        Vector::new(self, other)
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Direction {
    Vector(Rational),
    VectorNeg(Rational),
    Up,
    Down,
    Left,
    Right
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Vector {
    direction: Direction,
    length: isize  // Manhatten distance will be fine, it's all relative
}

impl Vector {
    fn new(a: Coord, b: Coord) -> Vector {
        let numerator = b.x - a.x;
        let denominator = b.y - a.y;

        if numerator == 0 && denominator == 0 {panic!("should not happen")}

        let direction = if denominator == 0 {
                if numerator < 0 { Direction::Left } else { Direction::Right }
            } else if numerator == 0 {
                if denominator > 0 { Direction::Down } else { Direction::Up }
            } else {
                if numerator < 0 {
                    Direction::VectorNeg(Rational::new(numerator, denominator))
                } else{
                    Direction::Vector(Rational::new(numerator, denominator))
                }
            };

        Vector {
            direction: direction,
            length: denominator.abs() + numerator.abs()
        }
    }
}


fn main() {
    env_logger::init();
    let asteroids: Vec<Coord> =
        include_str!("input.txt")
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, c)| {
                match c {
                    '#' => vec![Coord::new(x as isize, y as isize)],
                    _ => vec![],
                }
            }
            )
            })
        .collect();

    // let a = Coord::new(2, 2);
    // let b = Coord::new(3, 4);
    // let c = Coord::new(1, 0);

    // trace!("***** {:?} {:?}", a.vector_to(b), a.vector_to(c)  );
    // return;

    //trace!("{}", los_blocked(Vector::new(Coord::new(), Coord::new())));

    let mut los_count_map: HashMap<Coord, i32> = HashMap::new();

    for asteroid in asteroids.iter().clone() {
        for other_asteroid in asteroids.iter().clone() {
            if asteroid != other_asteroid {
                let vector = asteroid.vector_to(*other_asteroid); // the line from a to b
                let mut los = true;
                for intercept in asteroids.iter().clone() {
                    if intercept != asteroid && intercept != other_asteroid {
                        let intercept_vector = asteroid.vector_to(*intercept);
                        if los_blocked(vector, intercept_vector) {
                            los = false;
                            break;
                        }
                    }
                }
                if los {*los_count_map.entry(*asteroid).or_insert(0) += 1;}
            }
        }
    }

    trace!("{:?}", los_count_map);

    let (laser_coord, count) = los_count_map.iter().max_by(|(_a, a), (_b, b)| a.cmp(b)).unwrap();

    println!("part 1 {:?} {}", laser_coord, count);
}

fn los_blocked(sight: Vector, other: Vector) -> bool{
    if other.length > sight.length { return false }
    if sight.direction == other.direction { return true }
    false
}
