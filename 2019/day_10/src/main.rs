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
    Up,
    Vector(Rational),
    Down
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Vector {
    direction: Direction,
    length: isize
}

impl Vector {
    fn new(a: Coord, b: Coord) -> Vector {
        let numerator = b.y - a.y;
        let denominator = b.x - a.x;
        let delta_x = b.x as f64 - a.x as f64;
        let delta_y = b.y as f64 - a.y as f64;
        Vector {
            direction: if denominator == 0 {
                if numerator > 0 {Direction::Up} else {Direction::Down}
                } else {
                    Direction::Vector(Rational::new(numerator, denominator))
                },
            length: (delta_x * delta_x + delta_y * delta_y).sqrt() as isize
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
    if sight.direction == other.direction {return true}
    false
}
