use std::collections::HashSet;

fn parse_1(input: &str) -> (HashSet<i64>, Vec<Vec<&str>>) {
    let mut lines = input.lines();

    let seeds = HashSet::from_iter(
        lines
            .next()
            .unwrap()
            .split(':')
            .next_back()
            .unwrap()
            .split_whitespace()
            .map(|seed| seed.parse().unwrap()),
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
            }
            None => {
                break;
            }
        }
    }

    (seeds, maps)
}

fn part_1(input: &str) -> i64 {
    let (seeds, maps) = parse_1(input);

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

fn parse_2(input: &str) -> (Vec<(i64, i64)>, Vec<Vec<&str>>) {
    let mut lines = input.lines();

    let mut seed_ranges: Vec<(i64, i64)> = Vec::new();
    let mut seed_input = lines
        .next()
        .unwrap()
        .split(':')
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|input| input.parse().unwrap());

    loop {
        match seed_input.next() {
            Some(start) => {
                seed_ranges.push((start, start + seed_input.next().unwrap() - 1));
            },
            None => {
                break;
            }
        }
    };

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
            }
            None => {
                break;
            }
        }
    }

    (seed_ranges, maps)
}

fn part_2(input: &str) -> i64 {
    let (seed_ranges, maps) = parse_2(input);

    let mut step = seed_ranges.clone();

    for map in &maps {
        let mut new_step: Vec<(i64, i64)> = Vec::new();

        for (seed_start, seed_end) in &step {
            let mut translated = false;

            for line in map {
                let mut line_iter = line.split_whitespace();

                let dest_start: i64 = line_iter.next().unwrap().parse().unwrap();
                let src_start: i64 = line_iter.next().unwrap().parse().unwrap();
                let interval_size: i64 = line_iter.next().unwrap().parse().unwrap();


                let src_end = src_start + interval_size - 1;

                if (*seed_start).max(src_start) > (*seed_end).min(src_end) {
                    continue;
                }

                let offset = dest_start - src_start;

                // I think this is incorrect, it doesn't reflect the whole situation
                // intervals should break into multiple parts when not intersecting
                // fully.
                if *seed_start >= src_start && *seed_end <= src_end {
                    new_step.push((seed_start + offset, seed_end + offset));
                } else if *seed_start < src_start && *seed_end <= src_end {
                    new_step.push((src_start + offset, seed_end + offset));
                } else if *seed_start >= src_start && *seed_end > src_end {
                    new_step.push((seed_start + offset, src_end + offset));
                } else {
                    new_step.push((src_start + offset, src_end + offset));
                }

                translated = true;
            }

            if !translated {
                new_step.push((*seed_start, *seed_end));
            }
        }

        step = new_step;
    }

    step.into_iter().map(|x| x.0).min().unwrap()
}

pub fn run() {
    let input = include_str!("input.txt");

    println!("day_05_1 - {}", part_1(input));
    println!("day_05_2 - {}", part_2(input));
}
