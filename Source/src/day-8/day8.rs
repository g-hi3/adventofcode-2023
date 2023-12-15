use std::{env, fs, path};
use crate::haunted_wasteland::{Network, Path};

mod haunted_wasteland;

fn main() {
    let mut args = env::args();
    _ = args.next();
    let file_path = args.next().unwrap();
    let file_path = path::Path::new(&file_path);
    let file_content = fs::read_to_string(file_path).unwrap();
    let network = Network::new(&file_content).unwrap();
    let starting_nodes = network.starting_nodes();
    let loop_lengths = starting_nodes
        .iter()
        .filter_map(|node| network.run_instructions(&node.name()))
        .map(|path| path.len() as u64)
        .collect::<Vec<u64>>();

    let mut least_common_multiple = 1_u64;
    for loop_length in loop_lengths {
        println!("Loop length {loop_length}");
        println!("lcm of {least_common_multiple} and {loop_length} is {}", lcm(least_common_multiple, loop_length));
        least_common_multiple = lcm(least_common_multiple, loop_length);
    }
    println!();
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b
}