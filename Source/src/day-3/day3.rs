// https://adventofcode.com/2023/day/3

// --- Day 3: Gear Ratios ---
// You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.
//
// It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.
//
// "Aaah!"
//
// You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.
//
// The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.
//
// The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)
//
// Here is an example engine schematic:
//
// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
// In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
//
// Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?

// 527521 was too low.

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
                Ok(file_content) => println!("Sum of part numbers is {}", SchematicPart::extract(&file_content).get_part_numbers().iter().sum::<u32>()),
                Err(error) => eprintln!("Can't read file {}!\n{error}", file_path.display())
            }
        }
    }
}
