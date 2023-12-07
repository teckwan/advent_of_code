fn part_1(input: &str) -> u32 {
    let mut lines = input.lines();

    let times = lines
        .next()
        .unwrap()
        .split(':')
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|time| time.parse::<u32>().unwrap());

    let distances = lines
        .next()
        .unwrap()
        .split(':')
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|distance| distance.parse::<u32>().unwrap());

    times.zip(distances).map(|(time, distance)| {
        let mut ways = 0;

        for hold in 1..time {
            let run = (time - hold) * hold;

            if run > distance {
                ways += 1;
            }
        }

        ways
    }).product()
}

fn part_2(input: &str) -> u64 {
    let mut lines = input.lines();

    let time: u64 = lines
        .next()
        .unwrap()
        .split(':')
        .next_back()
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();

    let distance: u64 = lines
        .next()
        .unwrap()
        .split(':')
        .next_back()
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();

    let mut ways = 0;

    for hold in 1..time {
        let run = (time - hold) * hold;

        if run > distance {
            ways += 1;
        }
    }

    ways
}

pub fn run() {
    let input = include_str!("input.txt");

    println!("day_06_1 - {}", part_1(input));
    println!("day_06_2 - {}", part_2(input));
}
