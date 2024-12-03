use std::{fs::File, io::{BufRead, BufReader, Read}, path::PathBuf};

pub mod data_structures;

pub fn read_input_lines(year: u32, day: u32, part: &str) -> impl Iterator<Item = String> {
    let mut path = PathBuf::from(".\\input");
    path.push(year.to_string());
    path.push("day".to_string() + &day.to_string());
    path.push(format!("{}.in", part));

    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    lines.map(|x| x.unwrap())
}

pub fn read_input_to_string(year: u32, day: u32, part: &str) -> String {
    let mut path = PathBuf::from(".\\input");
    path.push(year.to_string());
    path.push("day".to_string() + &day.to_string());
    path.push(format!("{}.in", part));

    let mut file = File::open(path).unwrap();

    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();

    input
}