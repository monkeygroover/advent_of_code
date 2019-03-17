use std::io;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let sum = BufReader::new(io::stdin()).lines()
    .map(|x| x.unwrap().parse::<i32>())
    .fold(0,|a, b| a + b.unwrap());

    println!("{}", sum);
    Ok(())
}


