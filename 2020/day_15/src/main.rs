use log::*;

use std::collections::HashMap;

fn main() {
    env_logger::init();

    let input = vec![9,12,1,4,17,0,18];
    //let input = vec![0,3,6];
    let mut input_iter = input.iter();

    // remember the last two indices of each number spoken 
    let mut historical_spoken_data: HashMap<i32, [i32; 2]> = HashMap::new(); // turn_numbers of last two times this number was spoken
    
    let mut turn_number: i32 = 0;
    let mut last_number_spoken: i32 = 0; 

    let elves_iter = std::iter::from_fn(move || {
        turn_number += 1;
        debug!("");
        debug!("turn_number is {} last spoken was {}", turn_number, last_number_spoken);
        if let Some(&input_val) = input_iter.next() {
            // still reading the initial list, these are all the first time spoken
            historical_spoken_data.insert(input_val, [turn_number, 0]);
            last_number_spoken = input_val;
            debug!("reading input list, speaking {}", input_val);
            Some(input_val)
        } else {
            // get the indexes the last number was spoken,
            // it may have been spoken once or 2 or more times
            let previous_indices = historical_spoken_data.get(&last_number_spoken).unwrap();
            
            match previous_indices {
                [_, 0] => { // only spoken once before
                    debug!("{} last time was the first time, speaking 0", last_number_spoken);
                    last_number_spoken = 0;
                    debug!("speaking 0");
                    //get the turn_number of the last time zero was spoken
                    let previous_zero: i32 = historical_spoken_data.get(&0).map(|&[x, _]| x).unwrap_or(0);
                    historical_spoken_data.insert(0, [turn_number, previous_zero]);
                    Some(0)
                }
                [turn_last_spoken, turn_second_last_spoken] => {
                    let turn_delta = turn_last_spoken - turn_second_last_spoken;
                    debug!("{} was spoken before, {} - {} -> delta {}", last_number_spoken, turn_last_spoken, turn_second_last_spoken, turn_delta);

                    //get the turn_number (if exists) of the last time delta was spoken
                    let previous_delta: i32 = historical_spoken_data.get(&turn_delta).map(|&[x, _]| x).unwrap_or(0);
                    historical_spoken_data.insert(turn_delta, [turn_number, previous_delta]);
                    last_number_spoken = turn_delta;
                    debug!("speaking {}", turn_delta);
                    Some(turn_delta)
                }
            }
        }
    });

    // println!("{}", elves_iter.clone().skip(2020 - 1).next().unwrap());
    println!("{}", elves_iter.skip(30000000 - 1).next().unwrap());
}
