use grid::*;

use std::io::{stdout, Write};
use crossterm::{
    cursor::{MoveTo, Hide},
    execute,
    style::{style, Color, PrintStyledContent},
    terminal::{Clear, ClearType},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tile {
    Floor,
    Empty,
    Occupied
}

use std::{thread, time};

fn main() {
    execute!(stdout(), Clear(ClearType::All), Hide).unwrap();

    let lines = include_str!("input.txt").trim().lines();
    let stride = lines.clone().nth(0).map(|line| line.len()).unwrap();

    let tiles: Vec<Tile> = lines
    .flat_map(|line| {
        line.chars().map( |ch|
            match ch {
                '.' => Tile::Floor,
                'L' => Tile::Empty,
                '#' => Tile::Occupied,
                _ => unreachable!()
            })
    }).collect();

    let initial_grid = Grid::from_vec(tiles, stride);
    let height = initial_grid.rows();

    let mut current_grid = initial_grid;
    loop {
        draw_grid(&current_grid);
        let new_grid = step(&current_grid);
        if new_grid.flatten() == current_grid.flatten() {break};
        current_grid = new_grid;
        thread::sleep(time::Duration::from_millis(50));
    }
    
    let part_2 = current_grid.iter().filter(|&tile| *tile == Tile::Occupied).count();

    display_result(part_2, height);
}

fn step(input_grid: &Grid<Tile>) -> Grid<Tile> {
    let mut new_grid = input_grid.clone();

    for y in 0..input_grid.rows() {
        for x in 0..input_grid.cols() {
            let adj_count = count_visible_occupied((x,y), &input_grid);
            let new_tile = match input_grid[y][x] {
                Tile::Floor => Tile::Floor,
                Tile::Empty => if adj_count == 0 { Tile::Occupied } else { Tile::Empty },
                Tile::Occupied => if adj_count >= 5 { Tile::Empty } else { Tile::Occupied }
            };
            new_grid[y][x] = new_tile;
        }
    }

    new_grid
}

fn count_visible_occupied(pos: (usize, usize), grid: &Grid<Tile>) -> usize {
    let directions = vec![(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];
    directions.iter().fold(0, |acc, &direction| { 
        if occupied_seat_visible(pos, direction, &grid) { acc + 1 } else { acc }
    })
}

fn occupied_seat_visible(init_pos: (usize, usize), direction: (isize, isize), grid: &Grid<Tile>) -> bool {
    let width = grid.cols();
    let height = grid.rows();
    let (mut x, mut y) = init_pos;
    let (dx, dy) = direction;

    loop {
        // first check if we exit the grid
        if x == 0 && dx == -1 { return false }    
        if x == width - 1 && dx == 1 { return false }
        if y == 0 && dy == -1 { return false }    
        if y == height - 1 && dy == 1 { return false }

        x = (x as isize + dx) as usize;
        y = (y as isize + dy) as usize;

        match grid[y][x] {
            Tile::Occupied => return true,
            Tile::Empty => return false,
            Tile::Floor => ()
        }
    }
}

fn draw_grid(grid: &Grid<Tile>) -> () {
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            let tile: Tile = grid[y][x];
            let (colour, ch) = match tile {
                    Tile::Floor => (Color::Grey, '.'),
                    Tile::Empty => (Color::Green, 'L'),
                    Tile::Occupied => (Color::Red, '#')
                };

            execute!(stdout(), MoveTo(x as u16, y as u16), PrintStyledContent(style(ch).with(colour))).unwrap();
        }
    }
}

fn display_result(acc: usize, height: usize) -> () {
    execute!(stdout(), MoveTo(0, height as u16), PrintStyledContent(style(acc).with(Color::Cyan))).unwrap();
}
