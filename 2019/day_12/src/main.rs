use log::*;
use regex::Regex;

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
}

fn main() {
    env_logger::init();

    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();

    let mut moons: Vec<Moon> = include_str!("input.txt")
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

    //let imm_moons = moons.clone();

    for step in 0..1000 {
        trace!("step {}", step);
        for moon in moons.clone() {
            trace!("{:?}", moon);
        }

        let temp_moons = moons.clone();

        for moon in moons.iter_mut() {
            moon.update_velocity(&temp_moons)
        }

        for moon in moons.iter_mut() {
            moon.update_position()
        }
    }

    trace!("FINAL");
    for moon in moons.clone() {
        trace!("{:?}", moon);
    }

    let total_energy =  moons.iter().fold(0, |acc, moon| acc + moon.energy());

    println!("part1 {}", total_energy);
}
