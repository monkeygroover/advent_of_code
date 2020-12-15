use log::*;

use std::collections::HashMap;

fn main() {
    env_logger::init();

    let input = vec![9,12,1,4,17,0,18];
    //let input = vec![0,3,6];
    let mut input_iter = input.iter();

    // remember the last two indices of each number spoken 
    let mut historical_spoken_data: HashMap<i64, (Option<i64>, Option<i64>)> = HashMap::new(); // when was this last spoken?
    let mut index: i64 = 0;
    let mut last_number_spoken: i64 = 0; // last number spoken

    let elves_iter = std::iter::from_fn(move || {
        index += 1;
        debug!("");
        debug!("index is {} last spoken was {}", index, last_number_spoken);
        if let Some(&input_val) = input_iter.next() {
            // still reading the initial list, these are all the first time spoken
            historical_spoken_data.insert(input_val, (Some(index), None));
            last_number_spoken = input_val;
            debug!("reading input list, speaking {}", input_val);
            Some(input_val)
        } else {
            // get the indexes the last number was spoken,
            // it may have been spoken once or 2 or more times
            let previous_indices = historical_spoken_data.get(&last_number_spoken).unwrap();
            
            match previous_indices {
                (Some(_), None) => {
                    debug!("{} last time was the first time, speaking 0", last_number_spoken);
                    last_number_spoken = 0;
                    debug!("speaking 0 a");
                    //get the index of the last time zero was spoken
                    let (last_zero, _) = historical_spoken_data[&0];
                    historical_spoken_data.insert(0, (Some(index), last_zero));
                    Some(0)
                }
                (Some(last_index), Some(second_last_index)) => {
                    let delta = last_index - second_last_index;
                    debug!("{} was spoken before, {} - {} -> delta {}", last_number_spoken, last_index, second_last_index, delta);

                    //get the index of the last time delta was spoken
                    if historical_spoken_data.contains_key(&delta) {
                        let (last_delta, _) = historical_spoken_data[&delta];

                        historical_spoken_data.insert(delta, (Some(index), last_delta));
                        last_number_spoken = delta;
                        debug!("speaking {}", delta);
                        Some(delta)
                    } else {
                        historical_spoken_data.insert(delta, (Some(index), None));
                        last_number_spoken = delta;
                        debug!("speaking {}", delta);
                        Some(delta)
                    }
                },
                (None, _) => unreachable!()
            }
        }
    });

    println!("{}", elves_iter.clone().skip(2020 - 1).next().unwrap());
    println!("{}", elves_iter.skip(30000000 - 1).next().unwrap());
}
