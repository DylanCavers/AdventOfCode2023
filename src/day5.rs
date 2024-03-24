use std::{io::{self}, ops::Range, fs::read_to_string};

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

    fn reverse_lookup(&self, input: i64) -> i64 {
        for map in &self.map {
            let rev = input - map.delta;
            if map.range.contains(&rev) {
                return rev;
            }
        }
        input
    }
}

pub fn day5(file_path: &String) -> io::Result<()> {
    let contents_str = read_to_string(file_path)
        .expect("unable to open file")
        .split("\r\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    let parsed = parse(contents_str);
    part1(&parsed);
    part2(&parsed);

    Ok(())
}

pub fn parse(input: Vec<String>) -> Day5 {
    let seeds_ = input[0].split_once(": ").unwrap().1;
    let seeds_list: Vec<i64> = seeds_.split(' ').map(|x| x.parse().unwrap()).collect();

    let mut mapping: Vec<Mapping> = Vec::new();

    let mut curmap = Mapping::default();
    for line in input[2..].iter() {
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

fn part2(data: &Day5) {
    // let mut min = i64::MAX;

    //for seed_range in data.seeds.chunks(2) {
    //    for seed in seed_range[0]..seed_range[0] + seed_range[1] {
    //        let mut cur = seed;
    //        for map in &data.mapping {
    //            cur = map.apply_mapping(cur);
    //        }
    //        min = min.min(cur);
    //    }
    //}

    let seed_ranges = data.seeds
    .chunks(2)
    .map(|vec| Range {
        start: vec[0],
        end: vec[0] + vec[1],
    }).collect::<Vec<_>>();

    let mut location = 1_i64;
    loop {
        let mut cur = location;
        for map in data.mapping.iter().rev() {
            cur = map.reverse_lookup(cur);
        }
        for sr in &seed_ranges {
            if sr.contains(&cur) { 
                println!("Part 2: {}", location);
                return;
            }
        }
        location += 1;
    }

    // println!("Part 2: {}", min);
}