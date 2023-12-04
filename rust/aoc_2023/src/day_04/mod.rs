use std::collections::HashSet;

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|card| {
            let mut numbers = card[10..].split(" | ");

            let winning: HashSet<&str> =
                HashSet::from_iter(numbers.next().unwrap().split_whitespace());

            let owned_win: u32 = numbers
                .next()
                .unwrap()
                .split_whitespace()
                .filter(|number| winning.contains(number))
                .count()
                .try_into()
                .unwrap();

            if owned_win == 0 {
                0
            } else {
                2u32.pow(owned_win - 1)
            }
        })
        .sum()
}

pub fn run() {
    let input = include_str!("input.txt");

    println!("day_04_1 - {}", part_1(input))
}
