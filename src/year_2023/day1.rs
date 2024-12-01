use aoc::{data_structures::Trie, read_input_lines};

const YEAR: u32 = 2023;
const DAY: u32 = 1;

pub fn run(args: Vec<String>) {
    part_a(&args);
    part_b(&args);
}

fn part_a(_args: &Vec<String>) {
    let lines = read_input_lines(YEAR, DAY, "a");

    let mut numbers = Vec::new();
    
    for row in lines {
        let chars = row.chars();

        let mut digits = Vec::new();

        for character in chars {
            if let Some(digit) = character.to_digit(10) {
                digits.push(digit);
            }
        }

        numbers.push(10 * digits[0] + digits.pop().unwrap())
    }

    println!("Part a: {}", numbers.iter().fold(0, |n, i| n + i))
}

fn part_b(args: &Vec<String>) {
    let input_file = args[0].clone(); 

    let lines = read_input_lines(YEAR, DAY, &input_file);

    let mut digit_trie = Trie::new();

    let numbers = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    for number in numbers {
        digit_trie.insert(number, number);
    }

    let mut numbers = Vec::<u32>::new();

    for row in lines {
        let mut first = None;
        let mut last = None;

        let mut first_found = false;
        let mut last_found = false;

        for i in 0..=row.len() {
            if let Some(digit) = digit_trie.search(&row[i..]) {
                if !first_found {
                    first = Some(digit);
                    first_found = true;
                }
            }

            if let Some(digit) = digit_trie.search(&row[(row.len() - i)..]) {
                if !last_found {
                    last = Some(digit);
                    last_found = true;
                }
            }

            if first_found && last_found {
                break;
            }
        }

        // println!("{}", row);
        // println!("First: {:?}, Last: {:?}\n", first, last);

        let first = match_digit(&first.unwrap());
        let last = match_digit(&last.unwrap());

        numbers.push(10 * first + last);
    }

    println!("Part b: {}", numbers.iter().fold(0, |i, n| n + i))

}

fn match_digit(digit: &str) -> u32 {
    match digit {
        k if k == "one"     || k == "1" => 1,
        k if k == "two"     || k == "2" => 2,
        k if k == "three"   || k == "3" => 3,
        k if k == "four"    || k == "4" => 4,
        k if k == "five"    || k == "5" => 5,
        k if k == "six"     || k == "6" => 6,
        k if k == "seven"   || k == "7" => 7,
        k if k == "eight"   || k == "8" => 8,
        k if k == "nine"    || k == "9" => 9,
        k if k == "zero"    || k == "0" => 0,
        k => panic!("Invalid digit: {}", k)
    }
}