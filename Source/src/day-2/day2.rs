// https://adventofcode.com/2023/day/2

use std::{env, fs, path};
use cube_conundrum::*;

mod cube_conundrum;

fn main() {
    let mut command_args = env::args();
    _ = command_args.next();
    let first_arg = command_args.next();

    match first_arg {
        None => eprintln!("Invalid format! Run day2 like this:\n\n\tday2 <file path>"),
        Some(first_arg) => {
            let file_path = path::Path::new(&first_arg);

            if !file_path.exists() {
                eprintln!("Input file {} doesn't exist!", file_path.display());
                return;
            }

            let file_content = fs::read_to_string(&file_path);
            match file_content {
                Ok(file_content) => {
                    let game_id_sum = file_content
                        .lines()
                        .map(Game::from_line)
                        .filter_map(|game| match game {
                            Ok(game) => Some(game),
                            Err(_) => None
                        })
                        .map(|game| game.get_minimum_required_set())
                        .map(|cubes_set| cubes_set.power())
                        .sum::<u32>();
                    println!("Sum of minimum required sets' powers: {game_id_sum}")
                }
                Err(error) => eprintln!("Can't read input file {}: {error}", file_path.display())
            }
        }
    }
}