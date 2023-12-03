fn neighbor_symbol(x: usize, y: usize, map: &Vec<Vec<char>>) -> bool {
    let min_y = if y == 0 { y } else { y - 1 };
    let max_y = if y == map.len() - 1 { y } else { y + 1 };

    for j in min_y..=max_y {
        let line = &map[j];
        let min_x = if x == 0 { x } else { x - 1 };
        let max_x = if x == line.len() - 1 { x } else { x + 1 };

        for i in min_x..=max_x {
            let c = line[i];

            if !c.is_numeric() && c != '.' {
                return true
            }
        }
    }

    false
}

fn part_1(input: &str) -> u32 {
    let map =
        Vec::from_iter(input.lines().map(|line| Vec::from_iter(line.chars())));

    let mut total = 0;

    for y in 0..map.len() {
        let axis = &map[y];
        let mut number: u32 = 0;
        let mut adj_symbol = false;

        for x in 0..axis.len() {
            if map[y][x].is_numeric() {
                number = number * 10 + map[y][x].to_digit(10).unwrap();
                adj_symbol = adj_symbol || neighbor_symbol(x, y, &map);
            } else if number != 0 {
                if adj_symbol { total += number }

                number = 0;
                adj_symbol = false;
            }
        }

        if adj_symbol { total += number }
    }

    total
}

pub fn run() {
    let input = include_str!("input.txt");

    println!("day_03_1 - {}", part_1(input));
}
