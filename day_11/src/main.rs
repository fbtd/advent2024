#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

const MAIN_INPUT: &str = "input.txt";
const TEST_INPUT: &str = "test_input.txt";
const EXPECTED_TEST_RESULT_1: i32 = 55312;
const EXPECTED_TEST_RESULT_2: i32 = 55312;

fn solve_1(input_str: &str) -> i32 {
    let mut stones: Vec<u64> = input_str
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    //for _ in 0..1 {
    for blink in 0..25 {
        let original_len = stones.len();
        for i in 0..stones.len() {
            let i_this_stone = original_len - i - 1;
            let this_stone = stones[i_this_stone];

            if this_stone == 0 {
                stones[i_this_stone] = 1;
            } else {
                let digits = (this_stone as f64).log10();
                if digits % 2.0 >= 1.0 {
                    let left = this_stone / 10u64.pow((digits as u32) / 2 + 1);
                    let right = this_stone % 10u64.pow((digits as u32) / 2 + 1);
                    stones[i_this_stone] = left;
                    stones.push(right);
                    //stones.insert(i_this_stone +1, right);
                } else {
                    stones[i_this_stone] = this_stone * 2024;
                }
            }
            //dbg!(stones[i_this_stone]);
        }
        println!("{}", blink);
    }
    //dbg!(&stones);
    stones.len() as i32
}

fn blink(hm: HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut result: HashMap<u64, usize> = HashMap::new();
    for (n, how_many) in hm {
        let mut left: u64 = 1;
        let mut right: Option<u64> = None;
        if n > 0 {
            let digits = (n as f64).log10();
            if digits % 2.0 >= 1.0 {
                left = n / 10u64.pow((digits as u32) / 2 + 1);
                right = Some(n % 10u64.pow((digits as u32) / 2 + 1));
            } else {
                left = n * 2024;
            }
        }
        *result.entry(left).or_insert(0) += how_many;
        if right.is_some(){
            *result.entry(right.unwrap()).or_insert(0) += how_many;
        }
    }
    result
}

const MAX_LEVEL: u8 = 75;
fn solve_2(input_str: &str) -> u64 {
    let mut to_process: HashMap<u64, usize> = input_str
        .trim_end()
        .split(' ')
        .map(|s| (s.parse::<u64>().unwrap(), 1))
        .collect();
    for i in 0..MAX_LEVEL {
        to_process = blink(to_process.clone());
    }
    //dbg!(to_process);
    to_process.iter().map(|(k,v)| v).sum::<usize>() as u64
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_string = fs::read_to_string(MAIN_INPUT)?;
    let result_1 = solve_1(&input_string);
    println!("part_1: {result_1}");
    let result_2 = solve_2(&input_string);
    println!("part_2: {result_2}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input_string = fs::read_to_string(TEST_INPUT).unwrap();
        let result = solve_1(&input_string);
        assert_eq!(result, EXPECTED_TEST_RESULT_1);
    }

    #[test]
    fn test_part_2() {
        let input_string = fs::read_to_string(TEST_INPUT).unwrap();
        let result = solve_2(&input_string);
        assert_eq!(result, EXPECTED_TEST_RESULT_2 as u64);
    }
}
