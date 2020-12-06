use itertools::Itertools;
use std::fs;

fn main() {
    println!("Part 1: {}", expense_report(2));
    println!("Part 2: {}", expense_report(3));
}

fn expense_report(no_of_expenses: usize) -> usize {
    fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .split("\n")
        .map(|x| x.parse::<usize>().unwrap())
        .combinations(no_of_expenses)
        .filter(|x| x.iter().sum::<usize>() == 2020)
        .flatten()
        .fold(1, |x, y| x * y)
}
