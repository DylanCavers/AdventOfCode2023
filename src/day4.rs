use std::{collections::HashMap, fs::File, io::{self, Read}};

#[derive(Debug, PartialEq)]
pub struct Card {
    pub id: usize,
    pub winners: Vec<i32>,
    pub got: Vec<i32>,
}

pub fn day4(file_path: &String) -> io::Result<()> {
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
    let cards = parse(contents_str);
    let points = get_total_points(&cards);
    println!("Total points: {}", points);

    // Part 2
    let cards = better_parse(contents_str);
    let total_cards = get_total_cards(&cards);
    println!("Total card copies: {}", total_cards);

    Ok(())
}

fn parse(input: &str) -> Vec<HashMap<i32, bool>> {
    let mut cards: Vec<HashMap<i32, bool>> = Vec::new();

    for line in input.lines() {
        let mut card: HashMap<i32, bool>= HashMap::new();

        let parts: Vec<&str> = line.split(":").collect();
        let nums: Vec<&str> = parts[1].split("|").collect();

        let winners: Vec<&str> = nums[0].split_whitespace().collect();
        let winners_nums: Vec<i32> = winners.iter().map(|x| x.parse::<i32>().unwrap()).collect();

        let got: Vec<&str> = nums[1].split_whitespace().collect();
        let got_nums: Vec<i32> = got.iter().map(|x| x.parse::<i32>().unwrap()).collect();

        for num in got_nums {
            if winners_nums.contains(&num) {
                card.insert(num, true);
            } else {
                card.insert(num, false);
            }
        }

        cards.push(card);
    }

    cards
}

fn better_parse(input: &str) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    let mut count = 1;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        let nums: Vec<&str> = parts[1].split("|").collect();

        let winners: Vec<&str> = nums[0].split_whitespace().collect();
        let winners_nums: Vec<i32> = winners.iter().map(|x| x.parse::<i32>().unwrap()).collect();

        let got: Vec<&str> = nums[1].split_whitespace().collect();
        let got_nums: Vec<i32> = got.iter().map(|x| x.parse::<i32>().unwrap()).collect();

        let card = Card {
            id: count,
            winners: winners_nums,
            got: got_nums,
        };

        count += 1;
        cards.push(card);
    }

    cards
}

fn get_card_points(card: &HashMap<i32, bool>) -> i32 {
    let mut points = 0;

    for (_num, got) in card {
        if *got {
            if points == 0 {
                points += 1;
            } else {
                points *= 2;
            }
        }
    }

    points
}

fn get_total_points(cards: &Vec<HashMap<i32, bool>>) -> i32 {
    let mut points = 0;

    for card in cards {
        points += get_card_points(card);
    }

    points
}

fn get_total_cards(cards: &Vec<Card>) -> i32 {
    let total_cards = cards.len();
    let mut copies = vec![1; total_cards];

    println!("{:?}", copies);

    let mut cur_card = 0;

    for card in cards {
        let num_winners = get_num_winners(card); 

        for i in 0..num_winners {
            copies[(cur_card+1+i) as usize] += copies[cur_card as usize];
        }

        cur_card += 1;
    }

    let total = copies.iter().sum();

    total
}

fn get_num_winners(card: &Card) -> i32 {
    let mut num_winners = 0;

    for num in &card.winners {
        if card.got.contains(num) {
            num_winners += 1;
        }
    }

    num_winners
}