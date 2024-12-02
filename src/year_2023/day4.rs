use std::collections::HashSet;

use aoc::read_input_lines;

const YEAR: u32 = 2023;
const DAY: u32 = 4;

pub fn run(args: Vec<String>) {
    part_a(&args);
    part_b(&args);
}

fn part_a(args: &Vec<String>) {
    let lines = read_input_lines(YEAR, DAY, &args[0]);

    let mut sum = 0;

    for line in lines {
        let (winning_numbers, current_numbers) = parse_line(&line);

        let mut correct_numbers = 0;

        for number in current_numbers {
            if winning_numbers.contains(&number) {
                correct_numbers += 1;
            }
        }

        if correct_numbers >= 1 {
            sum += u32::pow(2, correct_numbers - 1);
        }
    }

    println!("Part a: {}", sum);
}

fn part_b(args: &Vec<String>) {
    let lines = read_input_lines(YEAR, DAY, &args[0]);

    let lines = lines.collect::<Vec<String>>();

    let mut card_totals = vec![1; lines.len()];

    for (i, line) in lines.into_iter().enumerate() {
        let (winning_numbers, current_numbers) = parse_line(&line);

        let mut correct_numbers = 0;

        for number in current_numbers {
            if winning_numbers.contains(&number) {
                correct_numbers += 1;
            }
        }

        for j in (i + 1)..=(i+correct_numbers) {
            card_totals[j] += card_totals[i];
        }
    }

    println!("Part b: {}", card_totals.iter().fold(0, |acc, i| acc + i))
}

fn parse_line(line: &str) -> (HashSet<u32>, Vec<u32>) {
    let mut chars = line.chars();

    while let Some(ch) = chars.next() {
        if ch == ':' {
            break;
        }
    }

    let numbers = chars.collect::<String>();

    let mut split = numbers.trim().split('|');

    let winning_numbers = split.next().unwrap().split(' ').filter(|x| x.len() > 0).map(|x| x.parse().unwrap()).collect::<HashSet<u32>>();
    let current_numbers = split.next().unwrap().split(' ').filter(|x| x.len() > 0).map(|x| x.parse().unwrap()).collect::<Vec<u32>>();

    (winning_numbers, current_numbers)
}