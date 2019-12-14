use log::*;

mod vm;
use crate::vm::{VM, State};

use std::{thread, time};

use std::io::{stdout, Write};
use crossterm::{
    cursor::{MoveTo, Hide},
    execute,
    style::{style, Color, PrintStyledContent},
    terminal::{Clear, ClearType},
};

const GRID_X: usize = 42;
const GRID_Y: usize = 24;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball
}

impl Tile {
    fn new(i: i64) -> Tile {
        match i {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("bad tile")
        }
    }
}

fn main() {
    env_logger::init();

    execute!(
        stdout(),
        Clear(ClearType::All),
        Hide
    ).unwrap();

    let mut initial_memory: Vec<i64> = include_str!("input.txt")
    .trim()
    .split(",")
    .map(|s| s.parse::<i64>().unwrap())
    .collect();

    initial_memory[0] = 2; // free play
    initial_memory.resize(1048576, 0);

    let mut game = VM::new("Arcanoid", initial_memory);

    let mut grid = vec![Tile::Empty; GRID_X * GRID_Y];
    let mut score: i64;

    let mut current_paddle_x: Option<i64> = None;
    let mut current_ball_x: Option<i64> = None;

    let mut outputs = vec![];

    loop {
        match game.run() {
            State::OutputProduced(val) => {
                outputs.push(val);
                if outputs.len() == 3 {
                    let x = outputs[0];
                    let y = outputs[1];

                    if x == -1 && y == 0 {
                        score = outputs[2];
                        display_score(score);
                    } else {
                        let tile = Tile::new(outputs[2]);
                        if tile == Tile::Ball {
                            current_ball_x = Some(x);
                            thread::sleep(time::Duration::from_millis(5));
                        }
                        if tile == Tile::Paddle {
                            current_paddle_x = Some(x);
                            thread::sleep(time::Duration::from_millis(5));
                        }
                        set_tile(x as usize, y as usize, tile, &mut grid);
                        display_tile(x, y, tile);
                    }

                    outputs.clear();
                }
            },
            State::InputPending => {
                let joy = move_joystick(current_ball_x.unwrap(), current_paddle_x.unwrap());
                game.handle_input(joy);
            },
            State::Halt => break,
            State::Continue => ()
        }
    }
}

fn move_joystick(ball_x: i64, paddle_x: i64) -> i64 {
    if paddle_x < ball_x {
        1
    } else if paddle_x > ball_x {
        -1
    } else {
        0
    }
}

fn set_tile(x: usize, y: usize, tile: Tile, grid: &mut Vec<Tile>) -> () {
    debug!("setting {}, {} to {:?}", x, y, tile);
    grid[y * GRID_X + x] = tile;
}

fn display_tile(x: i64, y: i64, tile: Tile) -> () {
    let (graphic, colour) = match tile {
            Tile::Empty => (' ', Color::Black),
            Tile::Wall => ('▓', Color::AnsiValue(172)),
            Tile::Block => ('▒', Color::AnsiValue(171)),
            Tile::Paddle => ('—', Color::Yellow),
            Tile::Ball => ('o', Color::White),
        };

    execute!(
        stdout(),
        MoveTo(x as u16, y as u16),
        PrintStyledContent(style(graphic).with(colour))
    ).unwrap();
}

fn display_score(score: i64) -> () {
    execute!(
        stdout(),
        MoveTo(0,23),
        PrintStyledContent(style(score).with(Color::Green))
    ).unwrap();
}
