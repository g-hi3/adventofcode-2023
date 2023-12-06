// https://adventofcode.com/2023/day/3

use std::{env, fs, path};
use crate::gear_ratios::{GetPartNumbers, SchematicPart};

mod gear_ratios;

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
                    let parts = SchematicPart::extract(&file_content);
                    println!("Sum of part numbers is {}", parts.get_part_numbers().iter().sum::<u32>());
                    println!("Sum of gear values is {}", parts.get_gear_values().iter().sum::<u32>());
                },
                Err(error) => eprintln!("Can't read file {}!\n{error}", file_path.display())
            }
        }
    }
}
