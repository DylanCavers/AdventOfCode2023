use std::fs::File;
use std::io::{self, BufRead};

pub fn day1(file_path: &String) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let calibration_val = get_calibration_value(line.as_str());
        total += calibration_val;
    }

    println!("Total: {}", total);

    Ok(())
}

pub fn day1part2(file_path: &String) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let names = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let calibration_val = get_real_calibration_value(line.as_str(), &names);
        total += calibration_val;
    }

    println!("Total: {}", total);

    Ok(())
}

fn get_calibration_value(text: &str) -> i64
{
    let digits: Vec<char> = text.chars().filter(|c| c.is_ascii_digit()).collect();

    format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
        .parse::<i64>()
        .unwrap()
}

fn get_real_calibration_value(text: &str, names: &[&str]) -> i64
{
    let mut replaced = text.to_owned();

    for (i, name) in names.iter().enumerate() {
        replaced = replaced.replace(name, format!("{}{}{}", name, i, name).as_str());
    }

    get_calibration_value(replaced.as_str())
}