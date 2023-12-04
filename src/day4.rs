use std::str::FromStr;
use itertools::Itertools;

struct Card {
    id: u32,
    winning: Vec<u32>,
    numbers: Vec<u32>,
    points: u32,
    matches: u32,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut winning = vec!();
        let mut numbers = vec!();
        let (card_id, nums) = line.split_once(":").unwrap();
        let (win_nums, our_nums) = nums.split_once("|").unwrap();

        for num in win_nums.split_ascii_whitespace() {
            winning.push(num.parse::<u32>().unwrap())
        }

        for num in our_nums.split_ascii_whitespace() {
            numbers.push(num.parse::<u32>().unwrap())
        }

        let mut matches = 0;
        numbers.iter().for_each(|n| {
            if winning.contains(n) {
                matches += 1;
            }
        });

        let points: u32 = if matches > 0 {
            2u32.pow(matches - 1)
        } else {
            0
        };

        let id = card_id.split_whitespace().last().unwrap().parse::<u32>().unwrap() - 1;

        Ok(Card{ id, winning, numbers, points, matches })
    }
}

fn solve_part1(text: String) -> u32 {
    text.lines()
        .map(Card::from_str)
        .map(|c| { c.unwrap().points })
        .sum()
}

fn solve_part2(text: String) -> u32 {
    let cards = text.lines()
        .map(Card::from_str)
        .map(|c| { c.unwrap() })
        .collect_vec();
    let mut copies: Vec<u32> = vec![1u32; cards.len()];
    cards.iter()
        .for_each(|c| {
            for _ in 0..*copies.get(c.id as usize).unwrap() {
                let mut new_cards = c.matches;
                while new_cards > 0 {
                    let idx = (c.id + new_cards) as usize;
                    if idx < copies.len() {
                        copies[idx] += 1;
                    }
                    new_cards -= 1
                }
            }
        });

    copies.iter().map(|i| {*i}).sum()
}

pub fn solve_day4() {
    println!("Day 4 Part 1 Solution: {}", solve_part1(read_day4_file()));
    println!("Day 4 Part 2 Solution: {}", solve_part2(read_day4_file()));
    println!();
}

fn read_day4_file() -> String {
    String::from(include_str!("../inputs/day4.txt"))
}

#[cfg(test)]
mod test {
    use crate::day4::{solve_part1, solve_part2};
    fn read_day4_test_file() -> String {
       String::from(include_str!("../inputs/day4_test.txt"))
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(read_day4_test_file()), 13)
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(read_day4_test_file()), 30)
    }


}