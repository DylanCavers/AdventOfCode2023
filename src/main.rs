#![allow(dead_code)]

use std::env;

mod day1;
use day1::day1part2;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Running with input file: {}", file_path);

    let _ = day1part2(file_path);
}