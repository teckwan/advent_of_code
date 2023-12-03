fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = line.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();

            first * 10 + last
        })
        .sum()
}

fn extract_word_to_digit(word: &str, reverse: bool) -> Option<u32> {
    if reverse {
        if word.ends_with("one") { Some(1) }
        else if word.ends_with("two") { Some(2) }
        else if word.ends_with("three") { Some(3) }
        else if word.ends_with("four") { Some(4) }
        else if word.ends_with("five") { Some(5) }
        else if word.ends_with("six") { Some(6) }
        else if word.ends_with("seven") { Some(7) }
        else if word.ends_with("eight") { Some(8) }
        else if word.ends_with("nine") { Some(9) }
        else { None }
    } else {
        if word.starts_with("one") { Some(1) }
        else if word.starts_with("two") { Some(2) }
        else if word.starts_with("three") { Some(3) }
        else if word.starts_with("four") { Some(4) }
        else if word.starts_with("five") { Some(5) }
        else if word.starts_with("six") { Some(6) }
        else if word.starts_with("seven") { Some(7) }
        else if word.starts_with("eight") { Some(8) }
        else if word.starts_with("nine") { Some(9) }
        else { None }
    }
}

fn get_first_digit(line: &str) -> u32 {
    let mut chars = line.chars();
    let mut a: Option<u32> = None;

    while a.is_none() {
        a = extract_word_to_digit(chars.as_str(), false);

        if a.is_none() {
            let c = chars.next().unwrap();
            a = if c.is_numeric() { c.to_digit(10) } else { None };
        }
    }

    return a.unwrap();
}

fn get_last_digit(line: &str) -> u32 {
    let mut chars = line.chars();
    let mut a: Option<u32> = None;

    while a.is_none() {
        a = extract_word_to_digit(chars.as_str(), true);

        if a.is_none() {
            let c = chars.next_back().unwrap();
            a = if c.is_numeric() { c.to_digit(10) } else { None };
        }
    }

    return a.unwrap();
}

fn part_2(input: &str) -> u32 {
    let mut total = 0;
    for line in input.split_whitespace() {
        total += get_first_digit(line) * 10 + get_last_digit(line);
    }

    total
}

pub fn run() {
    let input = include_str!("input.txt");

    println!("day_01_1 - {}", part_1(input));
    println!("day_01_2 - {}", part_2(input));
}
