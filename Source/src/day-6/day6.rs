// https://adventofcode.com/2023/day/6

use std::{env, fs, path};
use crate::wait_for_it::RaceRecord;

mod wait_for_it;

fn main() {
    let mut args = env::args();
    _ = args.next();
    let first_arg = args
        .next()
        .expect("Commands needs to be executed with a path parameter!");
    let file_path = path::Path::new(&first_arg);
    let file_content = fs::read_to_string(file_path)
        .expect(format!("Can't read file {}", file_path.display()).as_str());
    let my_toy_boat = wait_for_it::ToyBoat::get_my();
    let race_records = wait_for_it::RaceRecord::extract(&file_content);
    let product = race_records
        .iter()
        .map(|race_record| my_toy_boat.count_winning_hold_times(race_record))
        .reduce(|product, x| product * x)
        .expect("Unable to reduce product of all winning hold times!");
    println!("There are {product} possibilities to win all races.");
    let actual_race_record = RaceRecord::transform(race_records);
    let actual_winning_hold_times = my_toy_boat.count_winning_hold_times(&actual_race_record);
    println!("There are {actual_winning_hold_times} for the big race.");
}