use aoc::read_input_lines;
use itertools::Itertools;
// use std::fmt::Write;

const YEAR: u32 = 2024;
const DAY: u32 = 2;

pub fn run(args: Vec<String>) {
    part_a(&args);
    part_b(&args);
}

fn part_a(args: &Vec<String>) {
    let lines = read_input_lines(YEAR, DAY, &args[0]);

    let mut number_safe = 0;

    for line in lines {
        // println!("{}", line);

        let differences = line
            .split(' ')
            .map(|level| level.parse::<i32>().unwrap())
            .tuple_windows::<(i32, i32)>()
            .map(|(a, b)| a - b)
            .collect::<Vec<_>>();

        if validate(&differences) {
            number_safe += 1;
        }
    }

    println!("Part 1: {}", number_safe);
}

fn part_b(args: &Vec<String>) {
    let lines = read_input_lines(YEAR, DAY, &args[0]);

    let mut number_safe = 0;

    // let mut ok = false;
    // let mut buffer = String::new();

    'outer: for line in lines {
        // if !ok {
        //     println!("{}", buffer);
        // }

        // buffer = String::new();
        // ok = false;
        // writeln!(buffer, "{}", line);

        let differences = line
            .split(' ')
            .map(|level| level.parse::<i32>().unwrap())
            .tuple_windows::<(i32, i32)>()
            .map(|(a, b)| a - b)
            .collect::<Vec<_>>();

        // writeln!(buffer, "{:?}\n", differences);

        if validate(&differences) {
            number_safe += 1;
            // ok = true;
            continue 'outer;
        }

        // writeln!(buffer, "{:?}", &differences[1..]);

        for i in 1..differences.len() {
            let first = differences[i - 1];
            let second =  differences[i];

            let pre = &differences[..(i-1)];
            let post = &differences[(i+1)..]; 

            let mut attempt = Vec::new();

            attempt.extend(pre);
            attempt.push(first + second);
            attempt.extend(post);

            // writeln!(buffer, "{:?}", attempt);

            if validate(&attempt) {
                number_safe += 1;
                // ok = true;
                continue 'outer;
            }
        }

        // writeln!(buffer, "{:?}", &differences[..(differences.len() - 1)]);

        if validate(&differences[1..]) || validate(&differences[..(differences.len() - 1)]) {
            number_safe += 1;
            // ok = true;
            continue 'outer;
        }
    }

    // if !ok {
    //     println!("{}", buffer);
    // }

    println!("Part 2: {}", number_safe);
}

fn validate(differences: &[i32]) -> bool {
    for (current, next) in differences.iter().tuple_windows() {
        if *current == 0 || i32::signum(*current) != i32::signum(*next) || i32::abs(*current) > 3 {
            return false;
        }
    }

    if *differences.last().unwrap() == 0 || i32::abs(*differences.last().unwrap()) > 3 {
        return false;
    }

    true
}
