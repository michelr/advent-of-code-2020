use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

struct Line {
    min: i32,
    max: i32,
    character: char,
    password: String,
}

fn main() {
    let in_range = |line: Line| {
        let count = line.password.matches(line.character).count() as i32;
        line.min <= count && count <= line.max
    };

    let in_position = |line: Line| {
        let passsword_chars = line.password.chars().collect::<Vec<char>>();
        let is_first = passsword_chars[(line.min - 1) as usize] == line.character;
        let is_second = passsword_chars[(line.max - 1) as usize] == line.character;
        is_first != is_second
    };

    let part1: usize = valid_passwords(&in_range);
    let part2: usize = valid_passwords(&in_position);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn valid_passwords(f: &dyn Fn(Line) -> bool) -> usize {
    fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .split("\n")
        .map(parse)
        .map(f)
        .filter(|x| x == &true)
        .collect::<Vec<bool>>()
        .len()
}

fn parse(row: &str) -> Line {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<char>[a-z]): (?P<password>[a-z]+)$")
                .unwrap();
    }
    match RE.captures(&row) {
        Some(cap) => Line {
            min: cap.name("min").unwrap().as_str().parse::<i32>().unwrap(),
            max: cap.name("max").unwrap().as_str().parse::<i32>().unwrap(),
            character: cap.name("char").unwrap().as_str().chars().next().unwrap(),
            password: cap.name("password").unwrap().as_str().to_string(),
        },
        None => panic!("Regex capture problem"),
    }
}
