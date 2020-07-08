use std::collections::HashSet;

const SIZE: usize = 5;

#[derive(Debug, Default, Copy, Clone)]
struct Grid {
    squares: [[bool; SIZE]; SIZE],
}

impl Grid {
    pub fn parse(input: &str) -> Grid {
        let mut squares: [[bool; SIZE]; SIZE] = Default::default();

        for (x, line) in input.trim().lines().enumerate() {
            for (y, char) in line.chars().enumerate() {
                let value = match char {
                    '#' => true,
                    '.' => false,
                    _ => panic!("invalid input char")
                };

                squares[x][y] = value;
            }
        }

        Grid{ squares }
    }

    pub fn display(&self) -> () {
        for row in &self.squares {
            let row_string = row.iter().map(|&c| if c {'#'} else {'.'} ).collect::<String>();
            println!("{}", row_string);
        }
    }

    fn get_neighbour_count(&self, x: usize, y: usize) -> usize {
        let mut neighbours: Vec<bool> = Vec::new();
        if y > 0 {
            neighbours.push(self.squares[y - 1][x]);
        }
        if x > 0 {
            neighbours.push(self.squares[y][x - 1]);
        }
        if x + 1 < SIZE {
            neighbours.push(self.squares[y][x + 1]);
        }
        if y + 1 < SIZE {
            neighbours.push(self.squares[y + 1][x]);
        }

        neighbours.iter()
        .filter(|&x| *x)
        .count()
    }

    pub fn step(&mut self) -> () {
        let mut new_squares: [[bool; SIZE]; SIZE] = Default::default();
        for y in 0..SIZE {
            for x in 0..SIZE {
                let neighbour_bugs = self
                    .get_neighbour_count(x, y);

                    new_squares[y][x] = match (self.squares[y][x], neighbour_bugs) {
                    (true, 1) => true,
                    (true, _) => false,
                    (false, 1) | (false, 2) => true,
                    (false, _) => false,
                }
            }
        }
        self.squares = new_squares;
    }

    fn get_biodiversity_rating(&self) -> u32 {
        let (total, _) = self.squares.iter().flat_map(|r| r.iter())
            .fold( (0_u32,0_u32),|(total, power), element| {
                let new_total = total + if *element {2_u32.pow(power)} else {0};
                let new_power = power + 1;
                (new_total, new_power)
        });
        total
    }
}


fn main() {
    let mut grid = Grid::parse(include_str!("input.txt"));

    let mut ratings_seen = HashSet::new();
    
    loop {
        grid.step();
        let new_rating = grid.get_biodiversity_rating();
        if ratings_seen.contains(&new_rating) {break;}
        ratings_seen.insert(new_rating);
    } 

    grid.display();

    println!("{}", grid.get_biodiversity_rating());
}
