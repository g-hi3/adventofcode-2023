// https://adventofcode.com/2023/day/4

use std::{env, fs, path};
use crate::scratchcards::{get_copy_data, get_initial_copies, Scratchcard};

mod scratchcards;

fn main() {
    let mut command_args = env::args();
    _ = command_args.next();
    let first_arg = command_args.next();

    match first_arg {
        None => eprintln!("Invalid format! Run day2 like this:\n\n\tday2 <file path>"),
        Some(file_path) => {
            let file_path = path::Path::new(&file_path);

            if !file_path.exists() {
                eprintln!("Input file {} doesn't exist!", file_path.display());
                return;
            }

            match fs::read_to_string(&file_path) {
                Ok(file_content) => {
                    let scratchcards = Scratchcard::from_str(&file_content);
                    let mut total_scratchcards_count = scratchcards.len();
                    let mut copy_data = get_initial_copies(&scratchcards);

                    while copy_data.len() > 0 {
                        println!("{} + {}", total_scratchcards_count, copy_data.len());
                        total_scratchcards_count += copy_data.len();
                        copy_data = get_copy_data(&scratchcards, &copy_data);
                    }

                    println!("Total scratchcard cound is {total_scratchcards_count}");
                }
                Err(error) => eprintln!("File {} could not be read!\n{error}", file_path.display())
            }
        }
    }
}
