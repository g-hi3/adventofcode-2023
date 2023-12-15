// https://adventofcode.com/2023/day/6

use std::env;
use crate::camel_cards::Play;

mod camel_cards;

fn main() {
    let mut args = env::args();
    _ = args.next();
    let file_path = args.next().unwrap();
    let file_path = std::path::Path::new(&file_path);
    let file_content = std::fs::read_to_string(file_path).unwrap();
    let mut plays = Play::extract(&file_content);
    plays.sort();
    let ranks = plays
        .iter()
        .enumerate()
        .map(|(index, play)| {
            println!("{}: {} * ({}) = {}", play.hand(), play.bid(), (index + 1), play.bid() as u64 * (index as u64 + 1));
            play.bid() as u64 * (index as u64 + 1)
        })
        .sum::<u64>();
    println!("Ranks is {ranks}");
}