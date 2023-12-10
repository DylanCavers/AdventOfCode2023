#![allow(dead_code)]

use std::env;

mod day1;

mod day2;
use day2::day2part2;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Running with input file: {}", file_path);

    let _ = day2part2(file_path);
}