use std::{env, fs, path};
use crate::haunted_wasteland::Network;

mod haunted_wasteland;

fn main() {
    let mut args = env::args();
    _ = args.next();
    let file_path = args.next().unwrap();
    let file_path = path::Path::new(&file_path);
    let file_content = fs::read_to_string(file_path).unwrap();
    let network = Network::new(&file_content).unwrap();
    let path = network.run_instructions("AAA", "ZZZ").unwrap();
    println!("It took {} steps to reach 'ZZZ'.", path.len());
}