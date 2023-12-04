use itertools::Itertools;

#[derive(PartialEq, Debug, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn solve_part1(text: String) -> u32 {
    /* Find all the symbols */
    let symbol_positions = text.lines().enumerate().map(|(y, line)| {
        line.chars()
            .enumerate()
            .map(|(x, c)| {
                if !c.is_numeric() && c != '.' {
                    Some(Point { x, y })
                } else {
                    None
                }})
            .flatten()
            .collect::<Vec<Point>>()})
        .flatten()
        .collect::<Vec<Point>>();

    let num_lines = text.lines().count();
    /* Start building numbers */
    text.lines().enumerate().map(|(y, line)| {
        let mut num: u32 = 0;
        let mut touches_symbol = false;
        line.chars().enumerate().map(|(x, c)| {
            if c.is_numeric() {
                num = 10 * num + c.to_digit(10).unwrap();
                /* check symbol touch */
                if !touches_symbol {
                    get_positions(x, y, line.len(), num_lines).iter().for_each(|pos| {
                       if symbol_positions.contains(pos) {
                           touches_symbol = true;
                       }
                    });
                }
                if x == line.len() - 1 && touches_symbol {
                    // We found a number at the end of a line, return it.
                    num
                } else {
                    0 // return 0 as were not done with this number yet.
                }
            } else {
                // We are at a symbol so we can return the number (even if it's zero).
                let ret = if touches_symbol { num } else { 0 };
                num = 0;
                touches_symbol = false;
                ret
            }})
            .sum::<u32>()
    }).sum()
}

fn solve_part2(text: String) -> u32 {
    let gears = text.lines().enumerate().map(|(y, line)| {
        line.chars()
            .enumerate()
            .map(|(x, c)| {
                if c == '*' {
                    Some(Point { x, y })
                } else {
                    None
                }})
            .flatten()
            .collect::<Vec<Point>>()})
        .flatten()
        .collect::<Vec<Point>>();

    let num_lines = text.lines().count();
    let line_size = text.lines().last().unwrap().len();

    gears.iter()
        .map(|p| {
            get_positions(p.x, p.y, line_size, num_lines).iter()
                .map(|pos| { calculate_number(pos, &text) })
                .filter(|x| { *x > 0 })
                .unique()
                .collect::<Vec<u32>>()
        })
        .filter(|x| { x.len() == 2 })
        .map(|x| { x.into_iter().product::<u32>() })
        .sum()
}

fn calculate_number(pos: &Point, text: &String) -> u32 {
    let line = text.lines().nth(pos.y).unwrap();

    if !line.chars().nth(pos.x).unwrap().is_numeric() {
        return 0;
    }

    let mut left_num = vec!();
    let mut right_num = vec!();
    let (before, after) = line.split_at(pos.x);
    for c in before.chars().rev() {
        if !c.is_numeric() {
            break;
        }
        left_num.push(c.to_digit(10).unwrap());
    }

    for c in after.chars() {
        if !c.is_numeric() {
            break;
        }
        right_num.push(c.to_digit(10).unwrap());
    }

    let left_num : Vec<u32> = left_num.iter()
        .rev()
        .map(|x| { *x })
        .collect();
    let num = vec![left_num, right_num].concat();

    num.iter().fold(0, |acc, x| acc * 10 + x)
}

fn get_positions(x: usize, y: usize, x_max: usize, y_max: usize) -> Vec<Point> {
    let mut ret = vec!();
    /*
      ----------- First
      |       --- Second
      |   ----+-- Third
      |   |   |
      v   v   v
    |   |   |   |
    |   | * |   |
    |   |   |   |
     */
    if x > 0 {
        ret.push(Point { x: x - 1, y });
        if y > 0 {
            ret.push(Point { x: x - 1, y: y - 1 });
        }
        if y < y_max {
            ret.push(Point { x: x - 1, y: y + 1 });
        }
    }

    if x < x_max {
        ret.push(Point { x: x + 1, y });
        if y > 0 {
            ret.push(Point { x: x + 1, y: y - 1 });
        }
        if y < y_max {
            ret.push(Point { x: x + 1, y: y + 1 });
        }
    }

    if y > 0 {
        ret.push(Point { x, y: y - 1 })
    }

    if y < y_max {
        ret.push(Point { x, y: y + 1 })
    }

    ret
}

pub fn solve_day3() {
    println!("Day 2 Part 1 Solution: {}", solve_part1(read_day3_file()));
    println!("Day 2 Part 2 Solution: {}", solve_part2(read_day3_file()));
    println!();
}

fn read_day3_file() -> String {
    String::from(include_str!("../inputs/day3.txt"))
}

#[cfg(test)]
mod test {
    use crate::day3::{solve_part1, solve_part2};
    fn read_day3_test_file() -> String {
        String::from(include_str!("../inputs/day3_test.txt"))
    }

    #[test]
    fn test_part1() {
       assert_eq!(solve_part1(read_day3_test_file()), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(read_day3_test_file()), 467835);
    }

}