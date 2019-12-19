use log::*;

mod vm;
use crate::vm::{VM, State};

use std::io::{stdout, Write};
use crossterm::{
    cursor::{MoveTo, Hide},
    execute,
    style::{style, Color, PrintStyledContent},
    terminal::{Clear, ClearType},
};

#[derive(Copy, Clone, Debug, PartialEq)]
enum PullState {
    Unknown,
    Stationary,
    Pulled,
}

impl PullState {
    fn new(i: i64) -> PullState {
        match i {
            0 => PullState::Stationary,
            1 => PullState::Pulled,
            _ => panic!("bad pull state")
        }
    }
}

const GRID_X: i64 = 50;
const GRID_Y: i64 = 50;

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

    initial_memory.resize(1048576, 0);

    let mut grid = vec![PullState::Unknown; GRID_X as usize * GRID_Y as usize];

    // generate coord pairs
    for y in 0..GRID_Y {
        for x in 0..GRID_X {
            let state = run_bot(x, y, initial_memory.clone());
            set_tile(x, y, state, &mut grid);
        }
    }

    println!("part1: {}", grid.iter().filter(|g| **g == PullState::Pulled).collect::<Vec<&PullState>>().len());

    // part 2 scan along the bottom edge looking for the first square that fits
    for y in 100..10000 {
        // find x
        let mut x = y * 25 / 15;
        loop {
            if run_bot(x, y, initial_memory.clone()) == PullState::Pulled {
                break;
            }
            x +=1;
        }

        let test_bl = run_bot(x, y, initial_memory.clone());
        let test_tr = run_bot(x+99, y-99, initial_memory.clone());

        //println!("{}, {} {:?} {:?}", x, y, test_bl, test_tr);

        if test_bl == PullState::Pulled && test_tr == PullState::Pulled {
            println!("part2: {}", x * 10000 + y-99);
            return;
        }
    }
}

fn run_bot(x: i64, y: i64, initial_memory: Vec<i64>) -> PullState {
    let mut coords: Vec<i64> = vec![];
    coords.push(y);
    coords.push(x);

    let mut input = vec![];

    let mut bot = VM::new("Robot", initial_memory.clone());

    loop {
        match bot.run() {
            State::InputPending => {
                let coord = coords.pop().unwrap();
                bot.handle_input(coord);
                info!("InputPending; pushing {}", coord);
                input.push(coord);
            },
            State::OutputProduced(val) => {
                info!("OutputProduced({})", val);
                return PullState::new(val);
            },
            State::Halt => {info!("Halt")},
            State::Continue => ()
        }
    }
}


fn set_tile(x: i64, y: i64, pull: PullState, grid: &mut Vec<PullState>) -> () {
    debug!("setting {}, {} to {:?}", x, y, pull);
    grid[(y * GRID_X + x) as usize] = pull;
    display_tile(x, y, pull);
}

fn display_tile(x: i64, y: i64, pull: PullState) -> () {
    let (graphic, colour) = match pull {
            PullState::Unknown => (' ', Color::Black),
            PullState::Stationary => ('.', Color::AnsiValue(172)),
            PullState::Pulled => ('▓', Color::AnsiValue(171)),
        };

    execute!(
        stdout(),
        MoveTo(x as u16, y as u16),
        PrintStyledContent(style(graphic).with(colour))
    ).unwrap();
}
