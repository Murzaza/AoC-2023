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

pub fn solve_day1() {
   println!("Day 1 Part 1 Solution: {}", solve_d1p1(read_day1_file()));
   println!("Day 1 Part 2 Solution: {}", solve_d1p2(read_day1_file()));
   println!()
}

fn solve_d1p1(text: String) -> u32 {
   let mut sum = 0;
   for line in text.lines() {
      let mut nums = vec![];
      line.chars().for_each(|c| {
         if c.is_numeric() {
            nums.push(c.to_digit(10).unwrap())
         }
      });
      sum += get_calibration_value(&nums);
   }

   sum
}

fn solve_d1p2(text: String) -> u32 {
   //let text = read_da1_test_file();
   let mut sum = 0;
   for line in text.lines() {
      let new_line = convert_strings_to_num(line);
      let mut nums = vec![];
      new_line.chars().for_each(|c| {
         if c.is_numeric() {
            nums.push(c.to_digit(10).unwrap());
         }
      });
      sum += get_calibration_value(&nums);
   }

   sum
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

#[cfg(test)]
mod test {
   use crate::day1::{solve_d1p1, solve_d1p2};

   fn read_day1_part1_test_file() -> String {
     String::from(include_str!("../inputs/day1_part1_test.txt"))
   }

   fn read_day1_part2_test_file() -> String {
      let day1_test_file = include_str!("../inputs/day1_part2_test.txt");
      String::from(day1_test_file)
   }
   #[test]
   fn test_part1() {
      assert_eq!(solve_d1p1(read_day1_part1_test_file()), 142);
   }

   #[test]
   fn test_part2() {
      assert_eq!(solve_d1p2(read_day1_part2_test_file()), 281);
   }

}