use std::collections::HashMap;
use std::str::FromStr;
use itertools::Itertools;
use sscanf::sscanf;
use num::integer::lcm;

#[derive(Debug)]
struct Desert {
    adj_table: HashMap<String, (String, String)>,
    path: String,
    start: Vec<String>,
}

impl Desert {
    fn get_next_dir(&self, step: usize) -> char {
        self.path.chars().nth(step % self.path.len()).unwrap()
    }
}

impl FromStr for Desert {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (path, rest) = s.split_once("\n").unwrap();

        let adj_table: HashMap<String, (String, String)> = rest.trim()
            .lines()
            .map(|c| {
                let parsed = sscanf!(c, "{String} = ({String}, {String})");
                match parsed {
                    Ok( (key, left, right) ) => Some((key,(left, right))),
                    Err(..) => None,
                }})
            .flatten()
            .collect();

        let path = String::from(path.trim());

        let start = adj_table.keys()
            .map(|k| {
                if k.ends_with("A") {
                    Some(k.clone())
                } else {
                    None
                }
            })
            .flatten()
            .collect_vec();
        Ok( Desert { adj_table, path, start })
    }
}

fn solve_part1(text: String) -> usize {
    let desert = Desert::from_str(text.as_str()).unwrap();
    let mut steps = 0;
    let mut curr_key = String::from("AAA");

    while curr_key != String::from("ZZZ") {
        let (left, right) = desert.adj_table.get(curr_key.as_str()).unwrap();
        curr_key = match desert.get_next_dir(steps) {
            'L' => left.clone(),
            'R' => right.clone(),
            _ => panic!("LOL WUT"),
        };
        steps += 1;
    }

    steps
}

fn is_complete(values: &Vec<(String, usize)>) -> bool {
    values.into_iter()
        .fold(true, |acc, (k, _)| {
            acc & k.ends_with("Z")
        })
}

fn solve_part2(test: String) -> usize {
    let desert = Desert::from_str(test.as_str()).unwrap();
    let mut steps = desert.start.clone()
        .into_iter()
        .map(|s| {
            (s, 0usize)
        })
        .collect_vec();

    while !is_complete(&steps) {
        steps = steps.into_iter()
            .map(|(k, s)| {
                if k.ends_with("Z") {
                    (k, s)
                } else {
                    let (left, right) = desert.adj_table.get(k.as_str()).unwrap();
                    let key = match desert.get_next_dir(s) {
                        'L' => left.clone(),
                        'R' => right.clone(),
                        _ => panic!("LOL WUT"),
                    };
                    (key, s + 1)
                }
            })
            .collect_vec();
    }

    /* Get LCM */
    steps.into_iter()
        .map(|(k, s)| s)
        .fold(1, |acc, s| lcm(acc, s))
}

pub fn solve_day8() {
    println!("Day 8 Part 1 Solution: {}", solve_part1(read_day8_file()));
    println!("Day 8 Part 2 Solution: {}", solve_part2(read_day8_file()));
    println!();
}

fn read_day8_file() -> String {
    String::from(include_str!("../inputs/day8.txt"))
}


#[cfg(test)]
mod test {
    use crate::day8::{solve_part1, solve_part2};

    fn read_day8_test_file() -> String {
        String::from(include_str!("../inputs/day8_test.txt"))
    }

    fn read_day8_test_file_2() -> String {
        String::from(include_str!("../inputs/day8_test_2.txt"))
    }

    fn read_day8_test_file_3() -> String {
        String::from(include_str!("../inputs/day8_test_3.txt"))
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(read_day8_test_file()), 2);
        assert_eq!(solve_part1(read_day8_test_file_2()), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(read_day8_test_file_3()), 6)
    }
}