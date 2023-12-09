use std::backtrace::BacktraceStatus::Captured;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;
use itertools::Itertools;
use phf::phf_map;

const CARD_VALUE: phf::Map<char, u32> = phf_map!(
    'A' => 14u32,
    'K' => 13u32,
    'Q' => 12u32,
    'J' => 11u32,
    'T' => 10u32,
    '9' =>  9u32,
    '8' =>  8u32,
    '7' =>  7u32,
    '6' =>  6u32,
    '5' =>  5u32,
    '4' =>  4u32,
    '3' =>  3u32,
    '2' =>  2u32,
);

const CARD_VALUE_2: phf::Map<char, u32> = phf_map!(
    'A' => 14u32,
    'K' => 13u32,
    'Q' => 12u32,
    'J' =>  1u32,
    'T' => 10u32,
    '9' =>  9u32,
    '8' =>  8u32,
    '7' =>  7u32,
    '6' =>  6u32,
    '5' =>  5u32,
    '4' =>  4u32,
    '3' =>  3u32,
    '2' =>  2u32,
);

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Hand {
    hand: String,
    strength: u32,
    wildcard: u32,
    bid: u32,
}

fn calculate_strength(hand: &str) -> u32 {
    let mut cards = HashMap::new();

    hand.chars().for_each(|x| {
        let val = if cards.contains_key(&x) {
            cards.get(&x).unwrap() + 1
        } else {
            1
        };

        cards.insert(x, val);
    });

    let mut twos = 0;
    let mut has_three = false;

    let mut strength = 0;

    'finder: for k in cards.keys() {
        match *cards.get(k).unwrap() {
            5 => strength = 6,
            4 => strength = 5,
            3 => has_three = true,
            2 => twos += 1,
            _ => {}
        }
    }

    if strength == 0 {
        if has_three {
            if twos > 0 {
                strength = 4;
            } else {
                strength = 3;
            }
        } else if twos > 0 {
            if twos == 2 {
                strength = 2;
            } else {
                strength = 1;
            }
        }
    }

    strength
}

fn calculate_wildcard(hand: &str, strength: &u32) -> u32 {
    let wildcard = if hand.contains("J") {
        hand.chars()
            .sorted()
            .unique()
            .map(|c| {
                let mut new_hand = hand.replace("J", c.to_string().as_str());
                calculate_strength(new_hand.as_str())
            })
            .max()
            .unwrap()
    } else {
        *strength
    };

    if wildcard > *strength {
        wildcard
    } else {
        *strength
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(" ").unwrap();
        let hand = String::from(hand);
        let bid = bid.parse().unwrap();
        let strength = calculate_strength(hand.as_str());
        let wildcard = calculate_wildcard(hand.as_str(), &strength);

        Ok( Hand { hand, strength, bid, wildcard })
    }
}
struct CamelCards {
    hands: Vec<Hand>,
}

impl FromStr for CamelCards {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands = s.lines()
            .map(Hand::from_str)
            .flatten()
            .sorted_by(|a, b| {
                match a.strength.cmp(&b.strength) {
                    Ordering::Equal => {
                        for i in 0..5{
                            let a_val = *CARD_VALUE.get(&a.hand.chars().nth(i).unwrap()).unwrap();
                            let b_val = *CARD_VALUE.get(&b.hand.chars().nth(i).unwrap()).unwrap();
                            let ordering = a_val.cmp(&b_val);
                            if ordering != Ordering::Equal {
                                return ordering;
                            }
                        }
                        Ordering::Equal
                    },
                    v => { v },
                }
            })
            .collect_vec();

        Ok(CamelCards { hands })
    }
}


struct CamelCards2 {
    hands: Vec<Hand>,
}

impl FromStr for CamelCards2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands = s.lines()
            .map(Hand::from_str)
            .flatten()
            .sorted_by(|a, b| {
                match a.wildcard.cmp(&b.wildcard) {
                    Ordering::Equal => {
                        for i in 0..5{
                            let a_val = *CARD_VALUE_2.get(&a.hand.chars().nth(i).unwrap()).unwrap();
                            let b_val = *CARD_VALUE_2.get(&b.hand.chars().nth(i).unwrap()).unwrap();
                            let ordering = a_val.cmp(&b_val);
                            if ordering != Ordering::Equal {
                                return ordering;
                            }
                        }
                        Ordering::Equal
                    },
                    v => { v },
                }
            })
            .collect_vec();

        Ok(CamelCards2 { hands })
    }
}

fn solve_part1(text: String) -> u32 {
    CamelCards::from_str(text.as_str()).unwrap().hands.into_iter()
        .enumerate()
        .map(|(i, x)| (i as u32 + 1) * x.bid)
        .sum()
}

fn solve_part2(text: String) -> u32 {
    CamelCards2::from_str(text.as_str()).unwrap().hands.into_iter()
        .enumerate()
        .map(|(i, x)| (i as u32 + 1) * x.bid)
        .sum()
}

pub fn solve_day7() {
    println!("Day 7 Part 1 Solution: {}", solve_part1(read_day7_file()));
    println!("Day 7 Part 1 Solution: {}", solve_part2(read_day7_file()));
    println!();
}

fn read_day7_file() -> String {
    String::from(include_str!("../inputs/day7.txt"))
}
#[cfg(test)]
mod test {
    use itertools::assert_equal;
    use crate::day7::{solve_part1, solve_part2};

    fn read_day7_test_file() -> String {
        String::from(include_str!("../inputs/day7_test.txt"))
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(read_day7_test_file()), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(read_day7_test_file()), 5905);
    }
}