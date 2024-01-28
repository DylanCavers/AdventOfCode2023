use std::{collections::{HashMap, HashSet}, fs::File, io::{self, Read}};

#[derive(Debug, PartialEq)]
pub enum EngineSymbolType {
    Number(u32), // this will be th entire number, not just the digit
    Empty,
    Symbol(char),
}

#[derive(Debug, PartialEq)]
pub struct EngineSymbol {
    pub symbol_type: EngineSymbolType,
    pub id: usize, // for grouping Number() with different positions and the same value
}

pub fn day3(file_path: &String) -> io::Result<()> {
    
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => panic!("Failed to open file."),
    };

    // Read the file contents into a String
    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        panic!("Failed to read file.");
    }

    // Convert String to &str
    let contents_str: &str = &contents;

    
    let map = read_to_map(contents_str);
    let engine_symbol_map = get_engine_symbol_map(&map);

    // part 1
    let sum = sum_adjacent_numbers(&map, &engine_symbol_map);
    println!("Sum of adjacent numbers: {}", sum);

    // part 2
    let gear_ratios_sum = sum_gear_ratios(&map, &engine_symbol_map);
    println!("Sum of gear ratios: {}", gear_ratios_sum);

    Ok(())
}

pub fn read_to_map(input: &str) -> Map {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        width = row.len();
        map.push(row);
        height += 1;
    }

    Map {
        map,
        width,
        height,
    }
    
}

pub struct Map {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,   
}

pub fn get_engine_symbol_map(map: &Map) -> HashMap<(usize, usize), EngineSymbol> {
    let mut engine_symbols: HashMap<(usize, usize), EngineSymbol> = HashMap::new();
    let mut id = 0;

    for y in 0..map.height {
        let mut visited: HashSet<u32> = HashSet::new();
        for x in 0..map.width {
            // check if we've already visited this x coordinate
            // while gathering a digit
            if visited.contains(&(x as u32)) {
                continue;
            }

            let c = map.map[y][x];
            // check if c is a digit
            if c.is_digit(10) {
                let num: u32 = c.to_digit(10).unwrap();
                let mut x2 = x + 1;
                let mut digits: Vec<u32> = vec![num];

                // check if the next character is a digit
                while x2 < map.width {
                    let c2 = map.map[y][x2];
                    if c2.is_digit(10) {
                        // add the neighboring x coordinate to the visited set
                        visited.insert(x2 as u32);
                        let num = c2.to_digit(10).unwrap();
                        digits.push(num);
                        x2 += 1;
                    } else {
                        break;
                    }
                }
                // convert the digits vector into a single number
                let num = digits.iter().fold(0, |acc, x| acc * 10 + x);
                // iterate through the digits and add them to the hashmap
                for (i, _num) in digits.iter().enumerate() {
                    engine_symbols.insert(
                        (x + i, y),
                        EngineSymbol {
                            symbol_type: EngineSymbolType::Number(num),
                            id: id,
                        },
                    );
                }
                engine_symbols.insert(
                    (x, y),
                    EngineSymbol {
                        symbol_type: EngineSymbolType::Number(num),
                        id: id,
                    },
                );
            } else if c == '.' {
                engine_symbols.insert(
                    (x, y),
                    EngineSymbol {
                        symbol_type: EngineSymbolType::Empty,
                        id: id,
                    },
                );
            } else {
                engine_symbols.insert(
                    (x, y),
                    EngineSymbol {
                        symbol_type: EngineSymbolType::Symbol(c),
                        id: id,
                    },
                );
            }
            id += 1;
        }
    }

    engine_symbols
}

fn sum_adjacent_numbers(map: &Map, map_lookup: &HashMap<(usize, usize), EngineSymbol>) -> u32 {
    // iterate through map and check if the symbol is a number

    let mut visited_ids: HashSet<usize> = HashSet::new();
    let adjacent_neighbors: Vec<(i32, i32)> = vec![
        (0, 1),
        (1, 0),
        (1, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    let mut sum = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let symbol = map_lookup.get(&(x, y)).unwrap();
            if let EngineSymbolType::Number(num) = symbol.symbol_type {
                // don't sum the same number twice
                if visited_ids.contains(&symbol.id) {
                    continue;
                }
                // check if the symbol is adjacent to another symbol
                let mut adjacent = false;

                // iterate thorugh the adjacent neighbors and check if they are symbols
                for (x2, y2) in adjacent_neighbors.iter() {
                    let x2 = x2 + x as i32;
                    let y2 = y2 + y as i32;
                    if x2 < 0 || y2 < 0 {
                        continue;
                    }
                    if x2 >= map.width as i32 || y2 >= map.height as i32 {
                        continue;
                    }
                    if let Some(symbol) = map_lookup.get(&(x2 as usize, y2 as usize)) {
                        if let EngineSymbolType::Symbol(c) = symbol.symbol_type {
                            println!("{} at ({}, {}) is adjacent to {}", num, x, y, c);
                            adjacent = true;
                            break;
                        }
                    }
                }

                if adjacent {
                    sum += num;
                    visited_ids.insert(symbol.id);
                }
            }
        }
    }

    sum
}

fn sum_gear_ratios(map: &Map, map_lookup: &HashMap<(usize, usize), EngineSymbol>) -> u32 {
    let mut gear_ratios_sum: u32 = 0;
    let adjacent_neighbors: Vec<(i32, i32)> = vec![
        (0, 1),
        (1, 0),
        (1, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    for y in 0..map.height {
        for x in 0..map.width {
            if let Some(symbol) = map_lookup.get(&(x, y)) {
                if let EngineSymbolType::Symbol('*') = symbol.symbol_type {
                    let mut visited_ids: HashSet<usize> = HashSet::new();
                    let mut adjacent_numbers = 0;
                    let mut product = 1;

                    // check all the neighbors
                    for (x2, y2) in adjacent_neighbors.iter() {
                        let x3 = x2 + x as i32;
                        let y3 = y2 + y as i32;

                        if x3 >= 0 && y3 >= 0 && x3 < map.width as i32 && y3 < map.height as i32 {
                            if let Some(adjacent_symbol) =
                                map_lookup.get(&(x3 as usize, y3 as usize))
                            {
                                if visited_ids.contains(&adjacent_symbol.id) {
                                    continue;
                                }
                                if let EngineSymbolType::Number(num) = adjacent_symbol.symbol_type {
                                    adjacent_numbers += 1;
                                    product *= num;
                                    visited_ids.insert(adjacent_symbol.id); // don't count the same number twice
                                }
                            }
                        }
                    }

                    if adjacent_numbers == 2 {
                        gear_ratios_sum += product;
                    }
                }
            }
        }
    }

    gear_ratios_sum
}