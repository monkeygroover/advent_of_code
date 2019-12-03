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
}

fn main() {
    let directions = include_str!("input.txt")
                                .trim()
                                .lines()
                                .map(|line| {
                                    line.split(",")
                                });

    let re = Regex::new(r"(R|L|U|D)(\d+)").unwrap();

    let coord_sets: Vec<HashSet<Coord>> = directions.map(|wire| {
        println!("--------------");
        let mut coord_set = HashSet::new();
        let mut last_coord = Coord::new(0,0);
        for vector in wire {
            let caps = re.captures(vector).unwrap();
            let direction = caps[1].parse::<String>().unwrap().to_owned();
            let dist = caps[2].parse::<i32>().unwrap();

            let new_coord: Coord = match direction.as_ref() {
                "U" => {
                    // println!("U {}", dist);
                    let new_coord = Coord::new(last_coord.x, last_coord.y+dist);

                    for y in last_coord.y..=new_coord.y {
                        let fill_coord = Coord::new(last_coord.x, y);
                        if manhattan(fill_coord) != 0 {
                            coord_set.insert(fill_coord);
                            //println!("inserting {:?}", fill_coord);
                        }
                    };

                    new_coord
                },
                "D" => {
                    // println!("D {}", dist);
                    let new_coord = Coord::new(last_coord.x, last_coord.y-dist);

                    for y in new_coord.y..=last_coord.y {
                        let fill_coord = Coord::new(last_coord.x, y);
                        if manhattan(fill_coord) != 0 {
                            coord_set.insert(fill_coord);
                            // println!("inserting {:?}", fill_coord);
                        }
                    };
                    new_coord
                },
                "L" => {
                    // println!("L {}", dist);
                    let new_coord = Coord::new(last_coord.x-dist, last_coord.y);

                    for x in new_coord.x..=last_coord.x {
                        let fill_coord = Coord::new(x, last_coord.y);
                        if manhattan(fill_coord) != 0 {
                            coord_set.insert(fill_coord);
                            // println!("inserting {:?}", fill_coord);
                        }
                    };
                    new_coord
                },
                "R" => {
                    // println!("R {}", dist);
                    let new_coord = Coord::new(last_coord.x+dist, last_coord.y);

                    for x in last_coord.x..=new_coord.x {
                        let fill_coord = Coord::new(x, last_coord.y);
                        if manhattan(fill_coord) != 0 {
                            coord_set.insert(fill_coord);
                            // println!("inserting {:?}", fill_coord);
                        }
                    };
                    new_coord
                },
                _ => panic!("bad direction")
            };

            // println!("new coord: {:?}", new_coord);

            last_coord = new_coord;
        }
        coord_set
    }).collect();


    let intersections = coord_sets[0].intersection(&coord_sets[1]);
    let max = intersections.min_by(|a , b| manhattan(**a).cmp(&manhattan(**b))).unwrap();

    println!("{:?}", manhattan(*max));
}

fn manhattan(coord: Coord) -> i32 {
    coord.x.abs() + coord.y.abs()
}
