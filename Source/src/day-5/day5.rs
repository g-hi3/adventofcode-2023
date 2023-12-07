// https://adventofcode.com/2023/day/5

use std::env;
use crate::seed_fertilizer::Almanac;

mod seed_fertilizer;

fn main() {
    let mut args = env::args();
    _ = args.next();
    let first_argument = args.next().unwrap();
    let file_path = std::path::Path::new(&first_argument);
    let file_content = std::fs::read_to_string(file_path).ok().unwrap();
    let almanac = Almanac::new(&file_content).unwrap();
    almanac.find_lowest_location();
}