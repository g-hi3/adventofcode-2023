use std::{env, fs, path};
use crate::mirage_maintenance::History;

mod mirage_maintenance;

fn main() {
    let mut args = env::args();
    _ = args.next();
    let file_path = args.next().unwrap();
    let file_path = path::Path::new(&file_path);
    let file_content = fs::read_to_string(file_path).unwrap();
    let historical_data = History::extract(&file_content);
    let oasis_report_sum = historical_data.iter().map(|history| history.predict()).sum::<i64>();
    println!("OASIS report sum is {oasis_report_sum}");
}