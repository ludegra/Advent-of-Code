use std::collections::{HashMap, HashSet};

use aoc::read_input_lines;
use itertools::Itertools;

const YEAR: u32 = 2023;
const DAY: u32 = 5;

pub fn run(args: Vec<String>) {
    part_a(&args);
    part_b(&args);
}

fn part_a(args: &Vec<String>) {
    let mut lines = read_input_lines(YEAR, DAY, &args[0]);

    let seed_line = lines.next().unwrap();
    let mut seeds = seed_line[7..].split(' ').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();

    lines.next();

    // println!("{:?}", seeds);

    while let Some(_) = lines.next() {
        let mut map = HashMap::new();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            let split = line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();

            let start = split[1];
            let end = start + split[2];
            let skew = split[0] - start;

            assert!(map.insert((start, end), skew).is_none());
        }

        'seed_loop: for seed in seeds.iter_mut() {
            for ((range_start, range_end), skew) in map.iter() {
                if *range_start <= *seed && *seed < *range_end {
                    *seed += skew;
                    continue 'seed_loop;
                }
            }
        }

        // println!("{:?}", seeds);
    }

    println!("Part 1: {}", seeds.iter().fold(i64::MAX, |acc, seed| i64::min(acc, *seed)))
}

fn part_b(args: &Vec<String>) {
    let mut lines = read_input_lines(YEAR, DAY, &args[0]);

    let seed_line = lines.next().unwrap();
    let seeds = seed_line[7..].split(' ').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();

    let mut seed_ranges = seeds.into_iter().tuples().map(|(a, b)| (a, a + b)).collect::<HashSet<(i64, i64)>>();

    lines.next();

    // println!("{:?}", seed_ranges);

    while let Some(_) = lines.next() {
        let mut map = HashMap::new();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            let split = line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();

            let start = split[1];
            let end = start + split[2];
            let skew = split[0] - start;

            assert!(map.insert((start, end), skew).is_none());
        }

        let seed_ranges_old = seed_ranges;
        seed_ranges = HashSet::new();

        for range in seed_ranges_old.into_iter() {
            seed_ranges.extend(check_range_overlap(range, &map));
        }

        // println!("{:?}", seed_ranges);
    }

    let mut seed_ranges = Vec::from_iter(seed_ranges.into_iter());

    seed_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    
    // println!("{:?}", seed_ranges);

    println!("Part 2: {}", seed_ranges[0].0);
}

fn check_range_overlap(range: (i64, i64), captures: &HashMap<(i64, i64), i64>) -> HashSet<(i64, i64)> {
    let range_start = range.0;
    let range_end = range.1;

    if range_start >= range_end {
        return HashSet::new();
    }

    let mut result = HashSet::new();

    for ((capture_start, capture_end), modifier) in captures {
        if range_end <= *capture_start || *capture_end <= range_start {
            continue;
        }

        if *capture_start <= range_start {
            result.insert((range_start + modifier, i64::min(range_end, *capture_end) + modifier));

            let residual = (i64::min(range_end, *capture_end), range_end);

            result.extend(check_range_overlap(residual, captures)); 

            return result;
        }

        if range_end <= *capture_end {
            result.insert((i64::max(range_start, *capture_start) + modifier, range_end + modifier));

            let residual = (range_start, i64::max(range_start, *capture_start));

            result.extend(check_range_overlap(residual, captures));

            return result;
        }

        result.insert((i64::max(range_start, *capture_start), i64::min(range_end, *capture_end)));

        let residuals = vec![(range_start, i64::max(range_start, *capture_start)), (i64::min(range_end, *capture_end), range_end)];

        for residual in residuals {
            result.extend(check_range_overlap(residual, captures));
        }
    }

    result.insert(range);

    result
}