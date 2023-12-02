use regex::Regex;

struct Game {
    id: u32,
    valid: bool,
    power: u32,
}

fn solve_d2p1(text: String) -> u32 {
    text.lines()
        .map(|line| { parse_game(line) })
        .filter(|game| { game.valid })
        .map(|game| { game.id })
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

fn solve_d2p2(text: String) -> u32 {
    text.lines()
        .map(|line| { parse_game(line).power })
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

fn parse_game(line: &str) -> Game {
    let game_regex = Regex::new(r"Game (?<id>[0-9]+)").unwrap();
    let (game_text, checks) = line.split_once(":").unwrap();

    /* Get Game ID */
    let Some(ids) = game_regex.captures(game_text) else {
        panic!("Unable to parse game id: {game_text}");
    };

    let id = ids["id"].to_string().parse::<u32>().unwrap();

    let red_regex = Regex::new(r"(?<red>[0-9]+) red").unwrap();
    let green_regex = Regex::new(r"(?<green>[0-9]+) green").unwrap();
    let blue_regex = Regex::new(r"(?<blue>[0-9]+) blue").unwrap();

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    red_regex.captures_iter(checks).for_each(|cap| {
        let cap_num = cap["red"].to_string().parse::<u32>().unwrap();
        if cap_num > red {
            red = cap_num;
        }
    });

    green_regex.captures_iter(checks).for_each(|cap| {
        let cap_num = cap["green"].to_string().parse::<u32>().unwrap();
        if cap_num > green {
            green = cap_num;
        }
    });

    blue_regex.captures_iter(checks).for_each(|cap| {
        let cap_num = cap["blue"].to_string().parse::<u32>().unwrap();
        if cap_num > blue {
            blue = cap_num;
        }
    });

    let valid = if red <= 12 && green <= 13 && blue <= 14 {
        true
    } else {
        false
    };

    let power = red * green * blue;

    Game { id, valid, power }
}

pub fn solve_day2() {
    println!("Day 2 Part 1 Solution: {}", solve_d2p1(read_day2_file()));
    println!("Day 2 Part 2 Solution: {}", solve_d2p2(read_day2_file()));
    println!();
}

fn read_day2_file() -> String {
    let day1_test = include_str!("../inputs/day2.txt");
    String::from(day1_test)
}

#[cfg(test)]
mod test {
    use crate::day2::{solve_d2p1, solve_d2p2};

    fn read_day2_test_file() -> String {
        let day1_test_file = include_str!("../inputs/day2_test.txt");
        String::from(day1_test_file)
    }
    #[test]
    fn test_part1() {
        assert_eq!(solve_d2p1(read_day2_test_file()), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_d2p2(read_day2_test_file()), 2286)
    }
}