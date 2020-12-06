use std::fs;
use itertools::Itertools;

fn main() {
    println!("Part 1: {}", expense_report(2));
    println!("Part 2: {}", expense_report(3));
}

fn expense_report(no_of_expenses: usize) -> i32 {
    fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .combinations(no_of_expenses)
        .filter(|x| x.iter().sum::<i32>() == 2020)
        .flatten()
        .fold(1, |x, y| x * y)
}
