use std::env;
use std::fs;
use std::string;
use phf::phf_map;

const CONVERSION_MAP: phf::Map<&'static str, &'static str> = phf_map! {
   "one" => "1",
   "two" => "2",
   "three" => "3",
   "four" => "4",
   "five" => "5",
   "six" => "6",
   "seven" => "7",
   "eight" => "8",
   "nine" => "9",
   "zero" => "0",
};

pub fn solved_d1p1() {
   let text = read_day1_file();
   let mut sum = 0;
   for line in text.lines() {
      let mut nums = vec![];
      line.chars().for_each(|c| {
         if (c.is_numeric()) {
            nums.push(c.to_digit(10).unwrap())
         }
      });
      sum += get_calibration_value(&nums);
   }

   println!("Day1 Part 1 Answer: {}", sum);
}

pub fn solved_d1p2() {
   let text = read_day1_file();
   //let text = read_da1_test_file();
   let mut sum = 0;
   for line in text.lines() {
      let new_line = convert_strings_to_num(line);
      let mut nums = vec![];
      new_line.chars().for_each(|c| {
         if (c.is_numeric()) {
            nums.push(c.to_digit(10).unwrap());
         }
      });
      sum += get_calibration_value(&nums);
   }

   println!("Day1 Part 2 Answer: {}", sum);
}

fn get_calibration_value(nums: &Vec<u32>) -> u32 {
   let mut calibration_value = *nums.first().unwrap();
   calibration_value = calibration_value * 10 + *nums.last().unwrap();
   calibration_value
}

fn convert_strings_to_num(line: &str) -> String {
   let mut new_line = String::from(line);
   for entry in CONVERSION_MAP.entries() {
      let (key, value) = entry;
      let hits: Vec<_> = line.match_indices(key).collect();
      for hit in hits {
         let (idx, _) = hit;
         new_line.replace_range(idx..idx+1, value);
      }
   }
   new_line
}

fn read_day1_file() -> String {
   let day1_test = include_str!("../inputs/day1.txt");
   String::from(day1_test)
}

fn read_da1_test_file() -> String {
   let day1_test_file = include_str!("../inputs/day1_test.txt");
   String::from(day1_test_file)
}