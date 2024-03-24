use std::{fs::File, io::{self, Read}, ops::Range};

#[derive(Default, Debug, PartialEq)]
pub struct Day5 {
    seeds : Vec<i64>,
    mapping: Vec<Mapping>,
}

#[derive(Default, Debug, PartialEq)]
pub struct Mapping {
    map: Vec<SingleMap>,
}

#[derive(Default, Debug, PartialEq)]
pub struct SingleMap {
    range: Range<i64>,
    delta: i64,
}

impl Mapping {
    fn add_mapping(&mut self, dest: i64, source: i64, length: i64) {
        self.map.push(SingleMap {
            range: Range {
                start: source,
                end: source + length,
            },
            delta: dest - source,
        });
    }

    fn apply_mapping(&self, input: i64) -> i64 {
        for map in &self.map {
            if map.range.contains(&input) {
                return input + map.delta;
            }
        }
        input
    }
}

pub fn day5(file_path: &String) -> io::Result<()> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => panic!("Failed to open file."),
    };
    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        panic!("Failed to read file.");
    }
    let contents_str: &str = &contents;

    // Part 1
    let parsed = parse(contents_str.lines().map(String::from).collect()); 
    part1(&parsed);

    Ok(())
}


pub fn parse(input: Vec<String>) -> Day5 {
    let seeds_ = input[0].split_once(": ").unwrap().1;
    let seeds_list: Vec<i64> = seeds_.split(' ').map(|x| x.parse().unwrap()).collect();
    //println!("{:?}", seeds_list);

    let mut mapping: Vec<Mapping> = Vec::new();

    let mut curmap = Mapping::default();
    for line in input[3..].iter() {
        if line.is_empty() {
            continue;
        }
        if line.contains(':') {
            mapping.push(curmap);
            curmap = Mapping::default();
            continue;
        }
        let nums: Vec<i64> = line.split(' ').map(|x| x.parse().unwrap()).collect();
        curmap.add_mapping(nums[0], nums[1], nums[2]);
    }

    if !curmap.map.is_empty() {
        mapping.push(curmap);
    }
    
    Day5 {
        seeds: seeds_list,
        mapping,
    }
}

fn part1(data: &Day5) {
    let mut min = i64::MAX;
    for seed in &data.seeds {
        let mut cur = *seed;
        for map in &data.mapping {
            cur = map.apply_mapping(cur);
        }
        min = min.min(cur);
    }
    println!("Part 1: {}", min);
}