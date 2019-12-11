use log::*;

use std::{thread, time};

use std::sync::mpsc;
use std::sync::mpsc::Receiver;

mod vm;

use crate::vm::{VM, Input};

const GRID_X: usize = 48;
const GRID_Y: usize = 6;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum PanelColour {
    Unpainted,
    Black,
    White
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
    let (mut robot, input) = VM::new("A", initial_memory);
    let robot_thread = thread::spawn(move || {
        robot.run(Some(input_sender))
    });

    let mut grid = vec![PanelColour::Unpainted; GRID_X * GRID_Y];
    let mut robot_x = 0;
    let mut robot_y = 0;
    let mut robot_direction = Direction::Up;
    set_colour(robot_x, robot_y, PanelColour::White, &mut grid);

    loop {
        let input_val = match get_colour(robot_x, robot_y, &grid) {
            PanelColour::Unpainted => Input(0),
            PanelColour::Black => Input(0),
            PanelColour::White => Input(1),
        };

        let _unused = input.send(input_val);

        match wait_for_input(&input_receiver) {
            0 => set_colour(robot_x, robot_y, PanelColour::Black, &mut grid),
            1 => set_colour(robot_x, robot_y, PanelColour::White, &mut grid),
            99 => break, //TODO nicer way to exit
            x => panic!("bad input {}", x)
        };
        let (new_x, new_y, new_direction) = match wait_for_input(&input_receiver) {
            0 => turn_left(robot_x, robot_y, robot_direction),
            1 => turn_right(robot_x, robot_y, robot_direction),
            _ => panic!()
        };
        robot_x = new_x;
        robot_y = new_y;
        robot_direction = new_direction;
        display(&mut grid);
        thread::sleep(time::Duration::from_millis(10));
    }

    let _join_handle = robot_thread.join();
}

fn get_colour(x: usize, y: usize, grid: &Vec<PanelColour>) -> PanelColour {
    grid[y * GRID_X + x]
}

fn set_colour(x: usize, y: usize, colour: PanelColour, grid: &mut Vec<PanelColour>) -> () {
    debug!("setting {}, {} to {:?}", x, y, colour);
    grid[y * GRID_X + x] = colour;
}

fn turn_left(x: usize, y: usize, direction: Direction) -> (usize, usize, Direction) {
    match direction {
        Direction::Up    => (x - 1, y, Direction::Left),
        Direction::Down  => (x + 1, y, Direction::Right),
        Direction::Left  => (x, y + 1, Direction::Down),
        Direction::Right => (x, y - 1, Direction::Up),
    }
}

fn turn_right(x: usize, y: usize, direction: Direction) -> (usize, usize, Direction) {
    match direction {
        Direction::Up    => (x + 1, y, Direction::Right),
        Direction::Down  => (x - 1, y, Direction::Left),
        Direction::Left  => (x, y - 1, Direction::Up),
        Direction::Right => (x, y + 1, Direction::Down),
    }
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

fn display(grid: &mut Vec<PanelColour>) -> () {
    print!("{}[2J", 27 as char);
    let display: Vec<String> = grid.iter().map(|x| {
        match x {
            PanelColour::Unpainted => ' ',
            PanelColour::White     => '▒',
            PanelColour::Black     => ' ',
        }
    } )
    .collect::<Vec<char>>()
    .chunks(GRID_X)
    .map(|x| x.into_iter().collect())
    .collect::<Vec<String>>();

    for line in display {println!("{}", line)}
}
