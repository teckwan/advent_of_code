use std::collections::HashSet;

fn count_wins(card: &str) -> u32 {
    let mut numbers = card.split(':').next_back().unwrap().split(" | ");

    let winning: HashSet<&str> =
        HashSet::from_iter(numbers.next().unwrap().split_whitespace());

    numbers
        .next()
        .unwrap()
        .split_whitespace()
        .filter(|number| winning.contains(number))
        .count() as u32
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|card| {
            let wins = count_wins(card);

            if wins == 0 {
                0
            } else {
                2u32.pow(wins - 1)
            }
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    let cards = input.lines();

    let mut cards_count = vec![1; cards.clone().count()];

    for (i, card) in cards.enumerate() {
        let wins = count_wins(card);

        if wins > 0 {
            let start = i + 1;
            let end = (i + wins as usize).clamp(0, cards_count.len() - 1);

            for k in start..=end {
                cards_count[k] += cards_count[i]
            }
        }
    }

    cards_count.iter().sum()
}

pub fn run() {
    let input = include_str!("input.txt");

    println!("day_04_1 - {}", part_1(input));
    println!("day_04_2 - {}", part_2(input));
}
