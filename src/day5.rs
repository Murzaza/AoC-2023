use std::cmp::{max, min};
use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Range {
    val_start: u64,
    range: u64,
}

#[derive(Debug, Clone)]
struct Garden {
    seed_to_soil: HashMap<u64, Range>,
    soil_to_fertilizer: HashMap<u64, Range>,
    fertilizer_to_water: HashMap<u64, Range>,
    water_to_light: HashMap<u64, Range>,
    light_to_temp: HashMap<u64, Range>,
    temp_to_humid: HashMap<u64, Range>,
    humid_to_location: HashMap<u64, Range>,
    seeds: Vec<u64>,
}

fn parse_line(line: &str) -> (u64, Range) {
    let nums = line.split_whitespace().map(|x| x.parse().unwrap()).collect_vec();
    (nums[1], Range {val_start: nums[0], range: nums[2]})
}

fn get_value(num: u64, map: HashMap<u64, Range>) -> u64 {
    let mut key = None;
    for k in map.keys() {
        let range_top= *k + map.get(k).unwrap().range;
        let range_bot = *k;
        if range_bot <= num && range_top > num {
            key = Some(*k);
        }
    }

    let val = match key {
        Some(k) => map.get(&k).unwrap().val_start + (num - k),
        None => num,
    };

    val
}

impl Garden {
    fn get_locations_for_seeds(&self) -> Vec<u64> {
       self.seeds.iter()
           .map(|seed| { *seed })
           .map(|seed| { get_value(seed, self.seed_to_soil.clone()) })
           .map(|soil| { get_value(soil, self.soil_to_fertilizer.clone())})
           .map(|fertilizer| { get_value(fertilizer, self.fertilizer_to_water.clone()) })
           .map(|water| { get_value(water, self.water_to_light.clone()) })
           .map(|light| { get_value(light, self.light_to_temp.clone()) })
           .map(|temp| { get_value(temp, self.temp_to_humid.clone()) })
           .map(|humid| { get_value(humid, self.humid_to_location.clone()) })
           .collect_vec()
    }

    fn get_locations_for_seed_ranges(&self) -> Vec<u64> {
        self.seeds.iter()
            .map(|seed| { *seed })
            .enumerate()
            .map(|(i, start)| {
               if i % 2 == 0 {
                   let mut locations = vec!();
                   let range = self.seeds[i + 1];
                   for seed in start..start + range {
                       let soil = get_value(seed, self.seed_to_soil.clone());
                       let fertilizer = get_value(soil, self.soil_to_fertilizer.clone());
                       let water = get_value(fertilizer, self.fertilizer_to_water.clone());
                       let light = get_value(water, self.water_to_light.clone());
                       let temp = get_value(light, self.light_to_temp.clone());
                       let humid = get_value(temp, self.temp_to_humid.clone());
                       locations.push(get_value(humid, self.humid_to_location.clone()));
                   }
                   locations
               } else {
                   vec!()
               }
            })
            .flatten()
            .collect_vec()
    }
}

impl FromStr for Garden {
    type Err = ();

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        /* Parse seeds */

        let (seeds_txt, rest) = text.split_once("\n").unwrap();
        let (_, seed_nums) = seeds_txt.split_once(" ").unwrap();
        let seeds = seed_nums.split_whitespace()
            .map(|x| { x.parse::<u64>().unwrap() })
            .into_iter()
            .collect_vec();

        let rest = rest.lines()
            .filter(|x| { !x.is_empty() })
            .collect_vec();

        let mut maps = vec!();
        let map_regex = Regex::new(r"^([0-9]+) ([0-9]+) ([0-9]+)").unwrap();

        let mut curr_map = HashMap::new();

        for x in rest {
            if map_regex.is_match(x) {
                //Numbers
                let (k, v) = parse_line(x);
                curr_map.insert(k, v);
            } else {
                // Another map is starting
                if !curr_map.is_empty() { // We don't want to include the first empty map
                    maps.push(curr_map);
                    curr_map = HashMap::new();
                }
            }
        }

        if !curr_map.is_empty() {
            maps.push(curr_map);
        }

        assert_eq!(maps.len(), 7);
        /* Parse seed-to-soil */
        let seed_to_soil = (*maps.get(0).unwrap()).clone();

        /* Parse soil-to-fertilizer */
        let soil_to_fertilizer = (*maps.get(1).unwrap()).clone();

        /* Parse fertilizer-to-water */
        let fertilizer_to_water = (*maps.get(2).unwrap()).clone();

        /* Parse water-to-light */
        let water_to_light = (*maps.get(3).unwrap()).clone();

        /* Parse light-to-temp */
        let light_to_temp = (*maps.get(4).unwrap()).clone();

        /* Parse temp-to-humid */
        let temp_to_humid = (*maps.get(5).unwrap()).clone();

        /* Parse humid-to-location */
        let humid_to_location = (*maps.get(6).unwrap()).clone();

        Ok(Garden {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temp,
            temp_to_humid,
            humid_to_location,
            seeds
        })
    }
}

fn solve_part1(text: String) -> u64 {
    Garden::from_str(text.as_str())
        .unwrap()
        .get_locations_for_seeds()
        .into_iter()
        .min().unwrap()
}

fn solve_part2(text: String) -> u64 {
    Garden::from_str(text.as_str())
        .unwrap()
        .get_locations_for_seed_ranges()
        .into_iter()
        .min()
        .unwrap()
}

pub fn solve_day5() {
    println!("Day 5 Part 1 Solution {}", solve_part1(read_day5_file()));
    /* Not proud of my brute force solution */
    // println!("Day 5 Part 2 Solution {}", solve_part2(read_day5_file()));
    println!();
}

fn read_day5_file() -> String {
    String::from(include_str!("../inputs/day5.txt"))
}

#[cfg(test)]
mod test {
    use crate::day5::{solve_part1, solve_part2};

    fn read_day5_test_file() -> String {
        String::from(include_str!("../inputs/day5_test.txt"))
    }
    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(read_day5_test_file()), 35)
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(read_day5_test_file()), 46)
    }
}