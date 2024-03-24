#![allow(dead_code)]

use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Running with input file: {}", file_path);

    let _ = day5::day5(file_path);
}