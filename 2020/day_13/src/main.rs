use log::*;

fn main() {
    env_logger::init();

    let mut lines = include_str!("input.txt").trim().lines();

    let earliest_time = lines.next().map(|x| x.parse::<i32>().unwrap()).unwrap();
    let frequencies: Vec<i32> = lines.next()
        .map(|x| 
            x.split(",")
            .filter(|x| x.to_string() != "x")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        ).unwrap();

    debug!("{}, {:?}", earliest_time, frequencies);

    let (next_bus, wait) = frequencies.iter()
        .map(|f| (f, f - (earliest_time % f)))
        .min_by(|(_, x), (_, y)| x.cmp(y)).unwrap();

    println!("{} * {} = {}", next_bus, wait, next_bus * wait);
}
