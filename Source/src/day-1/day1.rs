// https://adventofcode.com/2023/day/1

use std::env;
use std::fs;
use std::path;

mod trebuchet;

fn main() {
    let mut command_args = env::args();
    _ = command_args.next();
    let first_arg = command_args.next();

    if let Some(file_path) = first_arg {
        let file_path = path::Path::new(&file_path);

        if !file_path.exists() {
            eprintln!("Input file {} doesn't exist!", file_path.display());
            return;
        }

        let file_content = fs::read_to_string(&file_path);

        match file_content {
            Ok(file_content) => {
                let calibration_result: u32 = file_content
                    .lines()
                    .map(|line| {
                        let calibration_value = trebuchet::extract_calibration_value(line);
                        println!("{line} -> {calibration_value:?}");
                        calibration_value
                    })
                    .sum();
                println!("Calibration result is {calibration_result}.");
            }
            Err(error) => eprintln!("Can't read input file {}: {error}", file_path.display()),
        }
    } else {
        eprintln!("Invalid format! Run day-1 like this:\n\n\tday1 <file path>");
    }
}