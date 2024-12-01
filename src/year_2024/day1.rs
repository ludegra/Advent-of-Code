use std::collections::HashMap;

use aoc::read_input_lines;
use regex::Regex;

const YEAR: u32 = 2024;
const DAY: u32 = 1;

pub fn run(args: Vec<String>) {
    part_a(&args);
    part_b(&args);
}

fn part_a(args: &Vec<String>) {
    let lines = read_input_lines(YEAR, DAY, &args[0]);

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in lines {
        let split = line.split(' ').filter(|s| !s.is_empty()).map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();

        left.push(split.first().unwrap().clone());
        right.push(split.last().unwrap().clone());
    }

    left.sort();
    right.sort();

    let mut sum = 0;

    for i in 0..left.len() {
        sum += i32::abs(left[i] - right[i]);
    }

    println!("Part 1: {}", sum)
}

fn part_b(args: &Vec<String>) {
    let lines = read_input_lines(YEAR, DAY, &args[0]);
    
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in lines {
        let split = line.split(' ').filter(|s| !s.is_empty()).map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();

        left.push(split.first().unwrap().clone());
        right.push(split.last().unwrap().clone());
    }

    let mut appearances: HashMap<i32, i32> = HashMap::new();

    for element in right {
        let entry = appearances.entry(element).or_default();

        *entry += 1;
    }

    let mut score = 0;

    for element in left {
        let entry = appearances.entry(element).or_default();

        score += element * *entry;
    }

    println!("Part 2: {}", score);
}