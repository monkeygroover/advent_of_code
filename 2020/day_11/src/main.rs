use std::io::{stdout, Write};
use crossterm::{
    cursor::{MoveTo, Hide},
    execute,
    style::{style, Color, PrintStyledContent},
    terminal::{Clear, ClearType},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Grid {
    Floor,
    Empty,
    Occupied
}

use std::{thread, time};

fn main() {
    execute!(
        stdout(),
        Clear(ClearType::All),
        Hide
    ).unwrap();

    let stride = include_str!("input.txt").trim().lines().nth(0).map(|line| line.len()).unwrap() as isize;

    let grid: Vec<Grid> = include_str!("input.txt").trim().lines()
    .flat_map(|line| {
        line.chars().map( |ch|
            match ch {
                '.' => Grid::Floor,
                'L' => Grid::Empty,
                '#' => Grid::Occupied,
                _ => unreachable!()
            })
    }).collect();

    let mut current_grid = grid;
    loop {
        let new_grid = step(&current_grid, stride);
        draw_grid(&&current_grid, &new_grid, stride);
        if new_grid == current_grid {break};
        current_grid = new_grid;
        thread::sleep(time::Duration::from_millis(100));
    }
    
    let part_1 = current_grid.iter().filter(|&tile| *tile == Grid::Occupied).count();

    display_result(part_1);
}

fn step(input: &Vec<Grid>, stride: isize) -> Vec<Grid> {
    let mut new_grid = Vec::with_capacity(input.len());

    for (i, g) in input.iter().enumerate() {
        let adj_count = count_adjacent_occupied(i, input, stride);
        let new_tile = match g {
            Grid::Floor => Grid::Floor,
            Grid::Empty => if adj_count == 0 { Grid::Occupied } else { Grid::Empty },
            Grid::Occupied => if adj_count >= 4 { Grid::Empty } else { Grid::Occupied }
        };
        new_grid.push(new_tile);
    }

    new_grid
}

fn count_adjacent_occupied(pos: usize, input: &Vec<Grid>, stride: isize) -> usize {
    //check the 8 positions 
    let mut offsets: Vec<isize> = vec![0-stride, stride]; //above and below
    if pos % stride as usize != 0 { //not left column
        offsets.append(&mut vec![0 - (stride + 1), -1, stride - 1]);
    }

    if pos % stride as usize != (stride - 1) as usize { //not right column
        offsets.append(&mut vec![1 - stride, 1, stride + 1]);
    }
    let offsets = offsets;

    offsets.iter().fold(0, |acc, offset| {
        let offset_pos = pos as isize + offset;

        if offset_pos < 0 || offset_pos >= input.len() as isize { 
            acc 
        } else {
            let offset_tile = input[offset_pos as usize];
            if offset_tile == Grid::Occupied {
                acc + 1
            } else {
                acc
            }
        }
    })
}

fn draw_grid(previous_grid: &Vec<Grid>, current_grid: &Vec<Grid>, stride: isize) -> () {
    for (i, (curr_g, prev_g)) in current_grid.iter().zip(previous_grid).enumerate() {
        if curr_g != prev_g {
            let (colour, ch) = match curr_g {
                    Grid::Floor => (Color::Grey, ' '),
                    Grid::Empty => (Color::Green, '░'),
                    Grid::Occupied => (Color::Red, '▓')
                };

            execute!(
                stdout(),
                MoveTo((i as u16) % stride as u16, (i / stride as usize) as u16),
                PrintStyledContent(style(ch).with(colour))
            ).unwrap();
        }
    }
}

fn display_result(acc: usize) -> () {
    execute!(
        stdout(),
        MoveTo(0,100),
        PrintStyledContent(style(acc).with(Color::Cyan))
    ).unwrap();
}
