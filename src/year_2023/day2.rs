use std::collections::HashMap;

use aoc::read_input_lines;

const YEAR: u32 = 2023;
const DAY: u32 = 2;

pub fn run(args: Vec<String>) {
    part_a(&args);
    part_b(&args);
}

fn part_a(args: &Vec<String>) {
    let lines = read_input_lines(YEAR, DAY, &args[0]);

    let mut accumulator = 0;

    let mut limits = HashMap::new();
    limits.insert("red", 12);
    limits.insert("green", 13);
    limits.insert("blue", 14);

    'outer: for line in lines {
        let (id, values) = parse_input(&line);
        accumulator += id;

        for set in values {
            for (color, limit) in limits.iter() {
                if let Some(value) = set.get(*color) {
                    if value > limit {
                        accumulator -= id;

                        // println!("Game {} is impossible", id);

                        continue 'outer;
                    }
                }
            }
        }
    }

    println!("Part a: {}", accumulator);
}

fn part_b(args: &Vec<String>) {
    let lines = read_input_lines(YEAR, DAY, &args[0]);

    let mut accumulator = 0; 

    for line in lines {
        let (_, values) = parse_input(&line);

        let mut minimums = HashMap::new();
        minimums.insert("red", 0);
        minimums.insert("blue", 0);
        minimums.insert("green", 0);

        for set in values {
            for (color, value) in set.iter() {
                let minimum_entry = minimums.get_mut(&color[..]).unwrap();
                *minimum_entry = u32::max(*minimum_entry, *value)
            }
        }
        
        let power = minimums.iter().fold(1, |acc, (_, min)| min * acc);

        accumulator += power;
    }

    println!("Part b: {}", accumulator);
}

fn parse_input(input: &str) -> (u32, Vec<HashMap<String, u32>>) {
    let mut chars = input.chars().skip(5);

    let mut id = String::new();

    while let Some(ch) = chars.next() {
        if ch == ':' {
            break;
        }

        id.push(ch);
    }

    let id: u32 = id.parse().unwrap();

    let input = chars.collect::<String>();

    let input = input.trim().split(';').map(|s| s.trim());

    let mut sets = Vec::new();

    for set in input {
        let mut set_counter = HashMap::new();

        for pile in set.split(',').map(|s| s.trim()) {
            let mut split = pile.split(' ');
            let number = split.next().unwrap().parse::<u32>().unwrap();
            let color = split.next().unwrap().to_string();

            set_counter.insert(color, number);
        }

        sets.push(set_counter);
    }


    (id, sets)
}