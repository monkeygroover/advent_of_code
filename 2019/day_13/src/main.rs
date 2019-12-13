use log::*;

use std::thread;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;

mod vm;

use crate::vm::{VM, Input};

const GRID_X: usize = 42;
const GRID_Y: usize = 23;

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

    let mut initial_memory: Vec<i64> = include_str!("input.txt")
    .trim()
    .split(",")
    .map(|s| s.parse::<i64>().unwrap())
    .collect();

    initial_memory.resize(1048576, 0);

    let (input_sender, input_receiver) = mpsc::channel::<Input>();
    let (mut game, _input) = VM::new("Arcanoid", initial_memory);
    let game_thread = thread::spawn(move || {
        game.run(Some(input_sender))
    });

    let mut grid = vec![Tile::Empty; GRID_X * GRID_Y];

    loop {
        let x = wait_for_input(&input_receiver);
        if x == 99 { break; } //TODO nicer way to exit
        let y = wait_for_input(&input_receiver);
        let tile = Tile::new(wait_for_input(&input_receiver));
        set_tile(x as usize, y as usize, tile, &mut grid);
    }

    display(&mut grid);

    let _join_handle = game_thread.join();

    println!("part1: {}", grid.iter().filter(|t| **t == Tile::Block).collect::<Vec<&Tile>>().len());

}


fn wait_for_input(input_receiver: &Receiver<Input>) -> i64 {
    match input_receiver.recv() {
        Ok(Input(value)) => {
            debug!("got {}", value);
            value
        }
        Err(_) => panic!("received junk")
    }
}

fn set_tile(x: usize, y: usize, tile: Tile, grid: &mut Vec<Tile>) -> () {
    debug!("setting {}, {} to {:?}", x, y, tile);
    grid[y * GRID_X + x] = tile;
}

fn display(grid: &mut Vec<Tile>) -> () {
    print!("{}[2J", 27 as char);
    let display: Vec<String> = grid.iter().map(|x| {
        match x {
            Tile::Empty => ' ',
            Tile::Wall => '▓',
            Tile::Block => '▒',
            Tile::Paddle => '-',
            Tile::Ball => 'o',
        }
    } )
    .collect::<Vec<char>>()
    .chunks(GRID_X)
    .map(|x| x.into_iter().collect())
    .collect::<Vec<String>>();

    for line in display {println!("{}", line)}
}
