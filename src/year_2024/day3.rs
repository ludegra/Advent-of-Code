use aoc::{read_input_lines, read_input_to_string};
use regex::{Regex, RegexBuilder};

const YEAR: u32 = 2024;
const DAY: u32 = 3;

pub fn run(args: Vec<String>) {
    part_a(&args);
    part_b(&args);
}

fn part_a(args: &Vec<String>) {
    let input = read_input_to_string(YEAR, DAY, &args[0]);

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let sum = re
        .captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| [a, b].into_iter().map(|x| x.parse::<u32>().unwrap()))
        .fold(0, |acc, factors| acc + factors.fold(1, |acc, x| acc * x));

    println!("Part 1: {}", sum);
}

fn part_b(args: &Vec<String>) {
    let input = read_input_to_string(YEAR, DAY, &args[0]);
    let input = input.replace('\n', "");

    let re_conditional = Regex::new(r"(?:^|do\(\))(.*?)(?:don't\(\)|$)").unwrap();
    let re_operation = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let sum = re_conditional
        .captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [inner])| re_operation.captures_iter(inner))
        .flatten()
        .map(|c| c.extract())
        .map(|(_, [a, b])| [a, b].into_iter().map(|x| x.parse::<u32>().unwrap()))
        .fold(0, |acc, factors| acc + factors.fold(1, |acc, x| acc * x));

    println!("Part 2: {}", sum)
}
