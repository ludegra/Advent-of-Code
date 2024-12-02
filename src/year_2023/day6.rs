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

    let times = re.split(&lines.next().unwrap()).skip(1).map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let rekords = re.split(&lines.next().unwrap()).skip(1).map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();

    let mut product = 1;

    for (time_total, rekord_distance) in zip(times, rekords) {
        let mut possible_inputs = 0;

        for time_held in 0..time_total {
            let distance = calculate_distance(time_held, time_total);

            if distance > rekord_distance {
                possible_inputs += 1;
            }
        }

        product *= possible_inputs;
    }

    println!("Part 1: {}", product);
}

fn calculate_distance(time_held: u64, time_total: u64) -> u64 {
    (time_total - time_held) * time_held
}

fn part_b(args: &Vec<String>) {
    let mut lines = read_input_lines(YEAR, DAY, &args[0]);

    let re = Regex::new(r"(\w+:)?\s+").unwrap();

    let time_total = re.replace_all(&lines.next().unwrap(), "").into_owned().parse::<u64>().unwrap();
    let rekord = re.replace_all(&lines.next().unwrap(), "").into_owned().parse::<u64>().unwrap();

    let optimal_hold = find_optimum(time_total);

    let mut sum = 0;

    for (t1, t2) in zip((0..optimal_hold).rev(), optimal_hold..time_total) {
        let left = calculate_distance(t1, time_total);
        let right = calculate_distance(t2, time_total);

        if left > rekord {
            sum += 1;
        }

        if right > rekord {
            sum += 1;
        }

        if left < rekord && right < rekord {
            break;
        }
    }

    println!("Part 2: {}", sum)
}

fn find_optimum(time_total: u64) -> u64 {
    time_total / 2
}