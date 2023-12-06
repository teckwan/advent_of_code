use std::collections::HashSet;

fn parse(input: &str) -> (HashSet<i64>, Vec<Vec<&str>>) {
    let mut lines = input.lines();

    let seeds = HashSet::from_iter(
        lines
            .next()
            .unwrap()
            .split(':')
            .next_back()
            .unwrap()
            .split_whitespace()
            .map(|seed| seed.parse().unwrap())
    );

    let mut maps: Vec<Vec<&str>> = Vec::new();

    loop {
        match lines.next() {
            Some(line) => {
                if line.eq("") {
                    continue;
                } else if line.ends_with("map:") {
                    maps.push(Vec::new());
                } else {
                    maps.last_mut().unwrap().push(line);
                }
            },
            None => {
                break;
            }
        }
    }

    (seeds, maps)
}

fn part_1(input: &str) -> i64 {
    let (seeds, maps) = parse(input);

    let mut step = seeds;

    for map in maps {
        let mut new_step: HashSet<i64> = HashSet::new();
        let mut translated: HashSet<i64> = HashSet::new();

        for line in map {
            let mut line_iter = line.split_whitespace();

            let dest_start: i64 = line_iter.next().unwrap().parse().unwrap();
            let src_start: i64 = line_iter.next().unwrap().parse().unwrap();
            let interval_size: i64 = line_iter.next().unwrap().parse().unwrap();

            let interval = src_start..=src_start + interval_size;

            for seed in &step {
                if interval.contains(seed) {
                    let offset = seed - src_start;
                    translated.insert(*seed);
                    new_step.insert(dest_start + offset);
                }
            }
        }
        new_step.extend(step.difference(&translated));
        step = new_step;
    }

    *Vec::from_iter(step).iter().min().unwrap()
}

fn part_2(input: &str) -> i64 {
}

pub fn run() {
    let input = include_str!("input.txt");

    println!("day_05_1 - {}", part_1(input));
    println!("day_05_2 - {}", part_2(input));
}
