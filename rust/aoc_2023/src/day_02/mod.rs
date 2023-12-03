fn part_1(games: &str, max_red: u8, max_green: u8, max_blue: u8) -> u32 {
    games
        .lines()
        .map(|game| {
            let mut x = game.split(": ");
            let header = x.next().unwrap();
            let mut to_add: u32 = header.replace("Game ", "").parse().unwrap();

            let grabs: Vec<&str> = x.next().unwrap().split("; ").collect();

            for grab in grabs {
                let valid = grab.split(", ").all(|color_count| {
                    let mut a = color_count.split(" ");

                    let count: u8 = a.next().unwrap().parse().unwrap();
                    let color: &str = a.next().unwrap();

                    match color {
                        "red" => count <= max_red,
                        "green" => count <= max_green,
                        "blue" => count <= max_blue,
                        _ => false,
                    }
                });

                if !valid {
                    to_add = 0;
                    break;
                }
            }

            to_add
        })
        .sum()
}

fn part_2(games: &str) -> u32 {
    games
        .lines()
        .map(|game| {
            let grabs: Vec<&str> =
                game.split(": ").last().unwrap().split("; ").collect();

            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for grab in grabs {
                for colors in grab.split(", ") {
                    let mut a = colors.split(" ");

                    let count: u32 = a.next().unwrap().parse().unwrap();
                    let color: &str = a.next().unwrap();

                    match color {
                        "red" => {
                            if count > max_red {
                                max_red = count
                            }
                        }
                        "green" => {
                            if count > max_green {
                                max_green = count
                            }
                        }
                        "blue" => {
                            if count > max_blue {
                                max_blue = count
                            }
                        }
                        _ => {}
                    }
                }
            }

            max_red * max_green * max_blue
        })
        .sum()
}

pub fn run() {
    let games = include_str!("input.txt");

    println!("day_02_1 - {}", part_1(games, 12, 13, 14));
    println!("day_02_2 - {}", part_2(games));
}
