use log::*;
use regex::Regex;

use num_integer::*;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Coord {
    x: i32,
    y: i32,
    z: i32
}

impl Coord {
    fn new(x: i32, y: i32, z: i32) -> Coord {
        Coord{x: x, y: y, z: z}
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Velocity {
    x: i32,
    y: i32,
    z: i32
}

impl Velocity {
    fn new() -> Velocity {
        Velocity{x: 0, y: 0, z: 0}
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Moon {
    pos: Coord,
    vel: Velocity
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon{pos: Coord::new(x, y, z), vel: Velocity::new()}
    }

    fn update_velocity(&mut self, moons: & Vec<Moon>) -> () {
        for moon in moons {
            if moon.pos.x > self.pos.x { self.vel.x +=1 }
            else if moon.pos.x < self.pos.x { self.vel.x -=1 }

            if moon.pos.y > self.pos.y { self.vel.y +=1 }
            else if moon.pos.y < self.pos.y { self.vel.y -=1 }

            if moon.pos.z > self.pos.z { self.vel.z +=1 }
            else if moon.pos.z < self.pos.z { self.vel.z -=1 }
        }
    }

    fn update_position(&mut self) -> () {
           self.pos.x += self.vel.x;
           self.pos.y += self.vel.y;
           self.pos.z += self.vel.z;
    }

    fn energy(self) -> i32 {
        let pot = self.pos.x.abs() +self.pos.y.abs() + self.pos.z.abs();
        let kin = self.vel.x.abs() +self.vel.y.abs() + self.vel.z.abs();
        pot * kin
    }

    fn cmp_x(self, other: Moon) -> bool {
        self.pos.x == other.pos.x && self.vel.x == other.vel.x
    }

    fn cmp_y(self, other: Moon) -> bool {
        self.pos.y == other.pos.y && self.vel.y == other.vel.y
    }

    fn cmp_z(self, other: Moon) -> bool {
        self.pos.z == other.pos.z && self.vel.z == other.vel.z
    }
}

fn main() {
    env_logger::init();

    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();

    let moons: Vec<Moon> = include_str!("input.txt")
    .trim()
    .lines()
    .map(|s| {
        let caps = re.captures(s).unwrap();
        let x = caps[1].parse::<i32>().unwrap();
        let y = caps[2].parse::<i32>().unwrap();
        let z = caps[3].parse::<i32>().unwrap();
        Moon::new(x, y, z)
    })
    .collect();

    let mut moons1 = moons.clone();

    for step in 0..1000 {
        trace!("step {}", step);
        for moon in moons1.clone() {
            trace!("{:?}", moon);
        }

        let temp_moons = moons1.clone();

        for moon in moons1.iter_mut() {
            moon.update_velocity(&temp_moons)
        }

        for moon in moons1.iter_mut() {
            moon.update_position()
        }
    }

    trace!("FINAL");
    for moon in moons1.clone() {
        trace!("{:?}", moon);
    }

    let total_energy = moons1.iter().fold(0, |acc, moon| acc + moon.energy());

    println!("part1 {}", total_energy);

    let mut cycle_x: Option<i64> = None;
    let mut cycle_y: Option<i64> = None;
    let mut cycle_z: Option<i64> = None;
    let mut step = 0;

    let mut moons2 = moons.clone();

    loop {
        step += 1;
        let temp_moons = moons2.clone();

        for moon in moons2.iter_mut() {
            moon.update_velocity(&temp_moons)
        }

        for moon in moons2.iter_mut() {
            moon.update_position()
        }

        if cycle_x == None {
            if moons.iter().zip(moons2.iter()).all(|(a, b)| a.cmp_x(*b)) {
                cycle_x = Some(step);
            }
        }

        if cycle_y == None {
            if moons.iter().zip(moons2.iter()).all(|(a, b)| a.cmp_y(*b)) {
                cycle_y = Some(step);
            }
        }

        if cycle_z == None {
            if moons.iter().zip(moons2.iter()).all(|(a, b)| a.cmp_z(*b)) {
                cycle_z = Some(step);
            }
        }

        if cycle_x != None && cycle_y != None && cycle_z != None { break }
    }

    println!("part2 {:?} {:?} {:?}", cycle_x, cycle_y, cycle_z);
    let lcm = cycle_x.unwrap().lcm(&cycle_y.unwrap()).lcm(&cycle_z.unwrap());
    println!("part2 {}", lcm);
}
