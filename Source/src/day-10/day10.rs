use std::{env, fs, path};

mod pipe_maze;

fn main() {
    let mut args = env::args();
    _ = args.next();
    let file_path = args.next().unwrap();
    let file_path = path::Path::new(&file_path);
    let file_content = fs::read_to_string(file_path).unwrap();
    let maze = pipe_maze::Maze::new(&file_content);
    let loop_length = maze.loop_length().unwrap();
    println!("It takes {} steps to the farthest point.",
             if loop_length % 2 == 0 { loop_length / 2 }
             else { loop_length / 2 + 1 });
}