#![allow(unused_imports)]
#![allow(dead_code)]

use std::error::Error;
use std::fmt;
use std::fs;

mod part_1;
mod part_2;

const MAIN_INPUT: &str = "input.txt";
const TEST_INPUT: &str = "test_input.txt";
const EXPECTED_TEST_RESULT_1: i32 = 1320;
const EXPECTED_TEST_RESULT_2: i32 = 145;

fn main() -> Result<(), Box<dyn Error>> {
    let input_string = fs::read_to_string(MAIN_INPUT)?;
    let result_1 = part_1::solve(&input_string)?;
    println!("part_1: {result_1}");
    let result_2 = part_2::solve(&input_string)?;
    println!("part_2: {result_2}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input_string = fs::read_to_string(TEST_INPUT).unwrap();
        let result = part_1::solve(&input_string).unwrap();
        assert_eq!(result, EXPECTED_TEST_RESULT_1);
    }


    #[test]
    fn test_part_2() {
        let input_string = fs::read_to_string(TEST_INPUT).unwrap();
        let result = part_2::solve(&input_string).unwrap();
        assert_eq!(result, EXPECTED_TEST_RESULT_2);
    }
}
