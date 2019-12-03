use regex::Regex;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord{x: x, y: y}
    }

    fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

fn main() {
    let re = Regex::new(r"(R|L|U|D)(\d+)").unwrap();

    let directions = include_str!("input.txt")
                                .trim()
                                .lines()
                                .map(|line| {
                                    line.split(",").map(|raw_command| {
                                        let caps = re.captures(raw_command).unwrap();
                                        let direction = caps[1].parse::<String>().unwrap().to_owned();
                                        let dist = caps[2].parse::<i32>().unwrap();
                                        match direction.as_ref() {
                                            "U" => Direction::Up(dist),
                                            "D" => Direction::Down(dist),
                                            "L" => Direction::Left(dist),
                                            "R" => Direction::Right(dist),
                                            _   => panic!("bad direction")
                                        }
                                    })
                                });

    let coord_vecs: Vec<Vec<Coord>> = directions.map(|wire| {
        let mut prev_coord = Coord::new(0,0);
        let mut coord_vec = vec![prev_coord];

        for command in wire {
            prev_coord = match command {
                Direction::Up(dist) => {
                    let new_coord = Coord::new(prev_coord.x, prev_coord.y+dist);
                    for y in (prev_coord.y+1)..=new_coord.y { coord_vec.push(Coord::new(prev_coord.x, y)); };
                    new_coord
                },
                Direction::Down(dist) => {
                    let new_coord =Coord::new(prev_coord.x, prev_coord.y-dist);
                    for y in (new_coord.y..prev_coord.y).rev() { coord_vec.push(Coord::new(prev_coord.x, y)); };
                    new_coord
                },
                Direction::Left(dist) => {
                    let new_coord = Coord::new(prev_coord.x-dist, prev_coord.y);
                    for x in (new_coord.x..prev_coord.x).rev() { coord_vec.push(Coord::new(x, prev_coord.y)); };
                    new_coord
                },
                Direction::Right(dist) => {
                    let new_coord = Coord::new(prev_coord.x+dist, prev_coord.y);
                    for x in (prev_coord.x+1)..=new_coord.x { coord_vec.push(Coord::new(x, prev_coord.y)); };
                    new_coord
                }
            };
        }
        coord_vec
    }).collect();

    let wire_1_point_set: HashSet<Coord> = coord_vecs[0].iter().cloned().collect();
    let wire_2_point_set: HashSet<Coord> = coord_vecs[1].iter().cloned().collect();
    let intersections = wire_1_point_set.intersection(&wire_2_point_set).filter(|coord| **coord != Coord::new(0,0));
    let min = intersections.clone().min_by(|a , b| a.manhattan().cmp(&b.manhattan())).unwrap();

    println!("part 1: {:?}", min.manhattan() );

    // walk each wire looking for the intersection points
    let minimum_path = intersections.map( |intersection| {
        let path_1_len = coord_vecs[0].iter().cloned().position(|coord| coord == *intersection).unwrap();
        let path_2_len = coord_vecs[1].iter().cloned().position(|coord| coord == *intersection).unwrap();
        path_1_len + path_2_len
    } ).min().unwrap();

    println!("part 2: {:?}", minimum_path);
}
