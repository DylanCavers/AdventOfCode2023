use std::fs::File;
use std::io::{self, BufRead};

pub fn day2(file_path: &String) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let constants = vec![12,13,14];

    let mut answer = 0;

    for line in reader.lines() {
        let line = parse(line.unwrap().as_str());
        if line[1] <= constants[0] && line[2] <= constants[1] && line[3] <= constants[2] {
            // println!("{}", line[0]);
            answer += line[0];
        }
    }

    println!("Total: {}", answer);

    Ok(())
}

pub fn day2part2(file_path: &String) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut answer = 0;

    for line in reader.lines() {
        let line = parse(line.unwrap().as_str());
        let powerset = line[1] * line[2] * line[3];
        answer += powerset;
    }

    println!("Total: {}", answer);

    Ok(())
}

pub fn parse(_line: &str) -> Vec<i32> {
    // stores the id and the max of each color seen
    let mut data = vec![0,0,0,0];

    let games = _line.split(":").collect::<Vec<&str>>();
    data[0] += games[0].split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

    let games = games[1].split(";").collect::<Vec<&str>>();
    for game in games {
        let game = game.split(",").collect::<Vec<&str>>();
        for color in game {
            let color = color.split_whitespace().collect::<Vec<&str>>();
            if color[1] == "red" && color[0].parse::<i32>().unwrap() > data[1] {
                data[1] = color[0].parse::<i32>().unwrap();
            } else if color[1] == "green" && color[0].parse::<i32>().unwrap() > data[2] {
                data[2] = color[0].parse::<i32>().unwrap();
            } else if color[1] == "blue" && color[0].parse::<i32>().unwrap() > data[3]{
                data[3] = color[0].parse::<i32>().unwrap();
            }
        }
    }
    
    // println!("{:?}", data);

    data
}