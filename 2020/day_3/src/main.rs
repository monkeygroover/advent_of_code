use log::*;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Coord {
    x: u32,
    y: u32
}

impl Coord {
    fn new(x: u32, y: u32) -> Coord {
        Coord{x: x, y: y}
    }
}

fn main() {
    env_logger::init();

    let height = include_str!("input.txt")
    .trim()
    .lines()
    .count() as u32;

    let width = include_str!("input.txt")
    .trim()
    .lines()
    .nth(1).unwrap()
    .chars()
    .count() as u32;

    trace!("{}, {}", height, width);

    let trees: Vec<Coord> =
        include_str!("input.txt")
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, c)| {
                match c {
                    '#' => vec![Coord::new(x as u32, y as u32)],
                    _ => vec![],
                }
            }
            )
            })
        .collect();

    let tree_count = count_trees_on_path(3,1,trees, width, height);

    println!("part1 {}", tree_count);
}

fn count_trees_on_path(x_step: u32, y_step: u32, trees: Vec<Coord>, repeat: u32, height: u32) -> u32 {
    let mut tob_coord = Coord::new(0,0);
    let mut tree_count = 0;

    while tob_coord.y < height {
        tob_coord.y += y_step;
        tob_coord.x += x_step;
        tob_coord.x = tob_coord.x % repeat;

        if trees.iter().any(|tree| *tree == tob_coord) {
            tree_count+=1;
        }
    }

    tree_count
}
