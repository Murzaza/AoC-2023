use std::str::FromStr;
use itertools::Itertools;

struct Strat {
    speed: u64,
    time: u64,
}

impl Strat {
    fn get_distance(&self) -> u64 {
        self.speed * self.time
    }
}

struct Race {
    duration: u64,
    max_distance: u64,
}

impl Race {
    fn get_winning_strats(&self) -> Vec<u64> {
        (0..self.duration).map(|holding| {
                Strat { speed: holding, time: self.duration - holding }
            })
            .map(|strat| { strat.get_distance() })
            .filter(|d| { *d > self.max_distance })
            .collect_vec()
    }
}

fn parse_text_into_races(text: String) -> Vec<Race> {
    let time_line = text.lines().nth(0).unwrap();
    let distance_line = text.lines().nth(1).unwrap();

    let times = time_line.split_whitespace()
        .skip(1)
        .map(|s| { s.parse::<u64>().unwrap() })
        .collect_vec();

    let distances = distance_line.split_whitespace()
        .skip(1)
        .map(|s| { s.parse::<u64>().unwrap() })
        .collect_vec();

    assert_eq!(times.len(), distances.len());
    (0..times.len())
        .map(|i| {
            Race { duration: times[i], max_distance: distances[i] }
        })
        .collect_vec()
}

fn parse_text_into_race(text: String) -> Race {
    let time_line = text.lines().nth(0).unwrap();
    let distance_line = text.lines().nth(1).unwrap();

    let time = time_line.split_whitespace()
        .skip(1)
        .fold(String::new(), |acc, b| acc + b)
        .parse::<u64>()
        .unwrap();

    let distance = distance_line.split_whitespace()
        .skip(1)
        .fold(String::new(), |acc, b| acc + b)
        .parse::<u64>()
        .unwrap();

    Race { duration: time, max_distance: distance }
}

fn solve_part1(text: String) -> u64 {
    parse_text_into_races(text).iter()
        .map(|r| { r.get_winning_strats().len() as u64 })
        .product()
}

fn solve_part2(text: String) -> u64 {
    parse_text_into_race(text).get_winning_strats().len() as u64
}

pub fn solve_day6() {
    println!("Day 6 Part 1 Solution {}", solve_part1(read_day6_file()));
    println!("Day 6 Part 2 Solution {}", solve_part2(read_day6_file()));
    println!();
}

fn read_day6_file() -> String {
    String::from(include_str!("../inputs/day6.txt"))
}

#[cfg(test)]
mod test {
    use crate::day6::{solve_part1, solve_part2};

    fn read_day6_test_file() -> String {
        String::from(include_str!("../inputs/day6_test.txt"))
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(read_day6_test_file()), 288)
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(read_day6_test_file()), 71503)
    }
}