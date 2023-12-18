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

    let tile_loop = maze.find_loop();
    let mut is_inside_loop = false;
    let mut loop_count = 0_u64;
    let mut inside_count = 0_u64;
    let mut outside_count = 0_u64;
    for (y, row) in maze.tiles().iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let coords = pipe_maze::Coords::new(x, y);
            if tile_loop.contains(&coords) {
                loop_count += 1;

            } else if is_inside_loop {
                inside_count += 1;
            } else {
                outside_count += 1;
            }
        }

        is_inside_loop = false;
    }

    // 1203 is too high
    println!("{loop_count} in loop == {}?", tile_loop.len());
    println!("{loop_count} in loop + {inside_count} inside loop + {outside_count} outside loop");
    println!("{} == 19'600", loop_count + inside_count + outside_count);
}

enum Marker {
    Pipe,
    Outside,
    Inside
}