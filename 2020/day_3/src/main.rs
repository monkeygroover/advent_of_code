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

    let lines = include_str!("input.txt")
    .trim()
    .lines();

    let height = lines.clone().count() as u32;
    let width = lines.clone().nth(1).unwrap().chars().count() as u32;
    let grid_size = Coord::new(width, height);

    trace!("{}, {}", height, width);

    let trees: Vec<Coord> =
        lines.enumerate()
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

    let path_vectors = vec![Coord::new(1,1),
                            Coord::new(3,1),
                            Coord::new(5,1),
                            Coord::new(7,1),
                            Coord::new(1,2)];

    let counts = path_vectors.iter().map(|delta| count_trees_on_path(*delta, trees.clone(), grid_size));

    let part1 = counts.clone().nth(1).unwrap();
    let part2: u32 = counts.product();

    println!("{}, {}", part1, part2);
}

fn count_trees_on_path(delta: Coord, trees: Vec<Coord>, grid_size: Coord) -> u32 {
    let mut tob_coord = Coord::new(0,0);
    let mut tree_count = 0;

    while tob_coord.y < grid_size.y {
        tob_coord.y += delta.y;
        tob_coord.x += delta.x;
        tob_coord.x = tob_coord.x % grid_size.x;

        if trees.iter().any(|tree| *tree == tob_coord) {
            tree_count+=1;
        }
    }

    tree_count
}
