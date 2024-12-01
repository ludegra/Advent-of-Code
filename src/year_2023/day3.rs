use std::collections::HashSet;

use aoc::read_input_lines;

const YEAR: u32 = 2023;
const DAY: u32 = 3;

pub fn run(args: Vec<String>) {
    part_a(&args);
    part_b(&args);
}

fn part_a(args: &Vec<String>) {
    let lines = read_input_lines(YEAR, DAY, &args[0]);

    let (grid, symbol_indices) = parse_input(lines);

    let mut numbers = Vec::new();

    for (i, j) in symbol_indices {
        let min_y = if i == 0 { 0 } else { i - 1 };
        let max_y = usize::min(grid.len() - 1, i + 1);

        let min_x = if j == 0 { 0 } else { j - 1 };
        let max_x = usize::min(grid[0].len() - 1, j + 1);

        let mut points_of_interest = Vec::new();

        for i in min_y..=max_y {
            for j in min_x..=max_x {
                // println!("Checking for digits: ({}, {}): {}", i, j, grid[i][j]);

                if grid[i][j].is_digit(10) {
                    // println!("({}, {}): {} is a digit", i, j, grid[i][j]);
                    points_of_interest.push((i, j));
                }
            }
        }

        // println!("Points of interest: {:?}", points_of_interest);

        let mut checked_indices = HashSet::new();

        for (i, j) in points_of_interest {
            // println!("Point of interest: ({}, {})", i, j);
            if checked_indices.contains(&(i, j)) {
                continue;
            }

            let mut left_digits = Vec::new();
            let mut right_digits = Vec::new();

            for j in (0..j).rev() {
                // println!("Checking ({}, {}): {}", i, j, grid[i][j]);

                checked_indices.insert((i, j));

                if !grid[i][j].is_digit(10) {
                    break
                }

                left_digits.push(grid[i][j]);
            }

            for j in j..(grid[0].len()) {
                // println!("Checking ({}, {}): {}", i, j, grid[i][j]);

                checked_indices.insert((i, j));

                if !grid[i][j].is_digit(10) {
                    break
                }

                right_digits.push(grid[i][j]);
            }

            let mut number = String::new();

            number.extend(left_digits.iter().rev());
            number.extend(right_digits);

            let number: u32 = number.parse().unwrap();

            // println!("({}, {}): {}", i, j, number);

            numbers.push(number);
        }
        // println!()
    }

    println!("Part a: {}", numbers.iter().fold(0, |acc, i| acc + i))
}

fn part_b(args: &Vec<String>) {
    let lines = read_input_lines(YEAR, DAY, &args[0]);

    let (grid, symbol_indices) = parse_input(lines);

    let mut sum = 0;

    for (i, j) in symbol_indices {
        if grid[i][j] != '*' {
            continue;
        }

        let min_y = if i == 0 { 0 } else { i - 1 };
        let max_y = usize::min(grid.len() - 1, i + 1);

        let min_x = if j == 0 { 0 } else { j - 1 };
        let max_x = usize::min(grid[0].len() - 1, j + 1);

        let mut points_of_interest = Vec::new();

        for i in min_y..=max_y {
            for j in min_x..=max_x {
                if grid[i][j].is_digit(10) {
                    points_of_interest.push((i, j));
                }
            }
        }

        let mut numbers = Vec::new();

        let mut checked_indices = HashSet::new();

        for (i, j) in points_of_interest {
            if checked_indices.contains(&(i, j)) {
                continue;
            }

            let mut left_digits = Vec::new();
            let mut right_digits = Vec::new();

            for j in (0..j).rev() {

                checked_indices.insert((i, j));

                if !grid[i][j].is_digit(10) {
                    break
                }

                left_digits.push(grid[i][j]);
            }

            for j in j..(grid[0].len()) {

                checked_indices.insert((i, j));

                if !grid[i][j].is_digit(10) {
                    break
                }

                right_digits.push(grid[i][j]);
            }

            let mut number = String::new();

            number.extend(left_digits.iter().rev());
            number.extend(right_digits);

            let number: u32 = number.parse().unwrap();

            numbers.push(number);
        }

        if numbers.len() != 2 {
            continue
        }

        sum += numbers[0] * numbers[1];
    }

    println!("Part b: {}", sum);
}

fn parse_input(input: impl Iterator<Item = String>) -> (Vec<Vec<char>>, Vec<(usize, usize)>) {
    let mut grid = Vec::new();
    let mut indices = Vec::new();

    for (i, line) in input.enumerate() {
        let mut row = Vec::new();

        for (j, ch) in line.chars().enumerate() {
            if !ch.is_digit(10) && ch != '.' {
                // println!("Symbol {} at ({}, {})", ch, i, j);
                indices.push((i, j));
            }

            row.push(ch);
        }
        grid.push(row);
    }

    (grid, indices)
}