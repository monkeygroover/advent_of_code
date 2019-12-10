use log::*;

use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord{x: x, y: y}
    }

    fn angle_distance(a: Coord, b: Coord) -> (f64, f64) {
        let delta_x = b.x as f64 - a.x as f64;
        let delta_y = b.y as f64 - a.y as f64;
        let angle = delta_y.atan2(delta_x);
        let length = (delta_x * delta_x + delta_y * delta_y).sqrt();
        (angle, length)
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
                    '#' => vec![Coord::new(x as i32, y as i32)],
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
                let vector = (asteroid, other_asteroid); // the line from a to b
                let mut los = true;
                for intercept in asteroids.iter().clone() {
                    if !has_los(vector, &intercept) {
                        los = false;
                        break;
                    }
                }
                if los {*los_count_map.entry(*asteroid).or_insert(0) += 1;}
            }
        }
    }

    trace!("{:?}", los_count_map);

    println!("part 1 {:?}", los_count_map.iter().max_by(|(_a, a), (_b, b)| a.cmp(b)).unwrap());
}


fn has_los(ends: (&Coord, &Coord), other: &Coord) -> bool{
    //ignore if other is either of the ends
    if ends.0 == other { return true }
    if ends.1 == other { return true }

    let (line_angle, line_dist) = Coord::angle_distance(*ends.0, *ends.1);
    let (intercept_angle, intercept_dist) = Coord::angle_distance(*ends.0, *other);

    if (intercept_dist - line_dist) > 0.001 { return true }
    if (intercept_angle - line_angle).abs() < 0.001 {return false}
    true
}
