use std::iter::zip;

use aoc::read_input_lines;
use regex::Regex;

const YEAR: u32 = 2023;
const DAY: u32 = 6;

pub fn run(args: Vec<String>) {
    part_a(&args);
    part_b(&args);
}

fn part_a(args: &Vec<String>) {
    let mut lines = read_input_lines(YEAR, DAY, &args[0]);

    let re = Regex::new(r"\s+").unwrap();

    let times = re.split(&lines.next().unwrap()).skip(1).map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
    let distances = re.split(&lines.next().unwrap()).skip(1).map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();

    for (time, distance) in zip(times, distances) {

    }
}

fn calculate_distance(time_held: u32, time_total: u32) -> u32 {
    (time_total - time_held) * time_held
}

fn part_b(args: &Vec<String>) {
    let lines = read_input_lines(YEAR, DAY, &args[0]);
}