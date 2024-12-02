use std::collections::HashMap;

use aoc::read_input_lines;

const YEAR: u32 = 2023;
const DAY: u32 = 7;

pub fn run(args: Vec<String>) {
    part_a(&args);
    part_b(&args);
}

const CARDS: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

fn part_a(args: &Vec<String>) {
    let mut hands = read_input_lines(YEAR, DAY, &args[0]).map(|line| {
        let mut split = line.split(' ');

        let hand = split.next().unwrap();
        let bid = split.next().unwrap().parse::<usize>().unwrap();

        let rating = rate_hand(hand);

        (rating, bid)
    }).collect::<Vec<_>>();

    hands.sort_by(|a, b| a.0.cmp(&b.0));

    println!("Part 1: {}", hands.into_iter().enumerate().fold(0, |acc, (rank, (_, bid))| acc + (rank + 1) * bid))
}

fn value_card(card: char) -> usize {
    CARDS.len() - CARDS.iter().position(|c| *c == card).unwrap()
}

fn rate_hand(hand: &str) -> [usize; 6] {
    let cards = hand.chars();

    let mut rating = [0; 6];

    let mut card_count: HashMap<char, usize> = HashMap::new();

    for (i, card) in cards.enumerate() {
        let count = card_count.entry(card).or_default();
        *count += 1;

        rating[i + 1] = value_card(card);
    }

    let mut card_count = card_count.into_iter().collect::<Vec<_>>();

    card_count.sort_by(|a, b| b.1.cmp(&a.1));

    card_count.push((' ', 0));

    // println!("{:?}", card_count);

    match (card_count[0].1, card_count[1].1) {
        k if k.0 == 5 => rating[0] = 6,
        k if k.0 == 4 => rating[0] = 5,
        k if k.0 == 3 && k.1 == 2 => rating[0] = 4,
        k if k.0 == 3 => rating[0] = 3,
        k if k.0 == 2 && k.1 == 2 => rating[0] = 2,
        k if k.0 == 2 => rating[0] = 1,
        _ => rating[0] = 0
    }

    rating
}

fn part_b(args: &Vec<String>) {
    let mut hands = read_input_lines(YEAR, DAY, &args[0]).map(|line| {
        let mut split = line.split(' ');

        let hand = split.next().unwrap();
        let bid = split.next().unwrap().parse::<usize>().unwrap();

        let rating = rate_hand_with_joker(hand);

        println!("{}: {:?}", hand, rating);

        (rating, bid)
    }).collect::<Vec<_>>();

    hands.sort_by(|a, b| a.0.cmp(&b.0));

    println!("Part 2: {}", hands.into_iter().enumerate().fold(0, |acc, (rank, (_, bid))| acc + (rank + 1) * bid))
}

fn value_card_with_joker(card: char) -> usize {
    if card != 'J' { CARDS.len() - CARDS.iter().position(|c| *c == card).unwrap() } else { 0 }
}

fn rate_hand_with_joker(hand: &str) -> [usize; 6] {
    let cards = hand.chars();

    let mut rating = [0; 6];

    let mut card_count: HashMap<char, usize> = HashMap::new();

    for (i, card) in cards.enumerate() {
        let count = card_count.entry(card).or_default();
        *count += 1;

        rating[i + 1] = value_card_with_joker(card);
    }

    let joker_count = *card_count.get(&'J').unwrap_or(&0);

    card_count.remove(&'J');

    let mut card_count = card_count.into_iter().collect::<Vec<_>>();

    card_count.sort_by(|a, b| b.1.cmp(&a.1));

    card_count.push((' ', 0));
    card_count.push(('\0', 0));

    // println!("{:?}", card_count);

    match (card_count[0].1, card_count[1].1) {
        k if k.0 + joker_count == 5 => rating[0] = 6,
        k if k.0 + joker_count == 4 => rating[0] = 5,
        k if k.0 == 3 && k.1 == 2 => rating[0] = 4,
        k if k.0 == 2 && k.1 + joker_count == 3 => rating[0] = 4,
        k if k.0 + joker_count == 3 => rating[0] = 3,
        k if k.0 == 2 && k.1 + joker_count == 2 => rating[0] = 2,
        k if k.0 + joker_count == 2 => rating[0] = 1,
        _ => rating[0] = 0
    }

    rating
}