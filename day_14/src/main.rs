#![allow(unused_imports)]
#![allow(dead_code)]

mod robot;
use robot::Robot;
use robot::RobotFactory;

use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

const MAIN_INPUT: &str = "input.txt";
const TEST_INPUT: &str = "test_input.txt";
const EXPECTED_TEST_RESULT_1: i32 = 12;
const EXPECTED_TEST_RESULT_2: i32 = 55312;

const MAX_TRIES: usize = 10_000;
const W: usize = 101;
const H: usize = 103;

fn print_robots(robots: &mut [Robot]) {
    for i in 1..MAX_TRIES {
        let mut bmp: [usize; W * H] = [0; W * H];
        for robot in robots.iter_mut() {
            robot.walk(1);
            bmp[(robot.px + robot.py * *robot.x as i32) as usize] += 1;
        }

        println!(" ============ {} ==============", i);
        let mut s = String::new();
        for (j, t) in bmp.into_iter().enumerate() {
            if j % W == 0 {
                s.push('\n')
            }
            if t > 9 {
                s.push('#');
            } else if t == 0 {
                s.push(' ');
            } else {
                s.push('o');
                //s.push_str(&t.to_string());
            }
        }
        print!("{s}\n");
        println!(" ==============================");
        println!("");
    }
}

fn solve_1(input_string: &str, grid_size: (usize, usize)) -> i32 {
    let rf = RobotFactory::new(grid_size.0, grid_size.1);
    let robots: Vec<Robot> = input_string.lines().map(|l| rf.new_robot(l)).collect();
    let mut robot_pools: [usize; 5] = [0; 5];
    for mut robot in robots {
        robot.walk(100);
        let q = robot.quadrant();
        if q > 0 {
            robot_pools[q] += 1;
        }
    }
    robot_pools.into_iter().fold(1, |acc, x| acc * max(x, 1)) as i32
}

fn solve_2(input_string: &str, grid_size: (usize, usize)) -> i32 {
    let rf = RobotFactory::new(grid_size.0, grid_size.1);
    let mut robots: Vec<Robot> = input_string.lines().map(|l| rf.new_robot(l)).collect();
    print_robots(&mut robots);
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_string = fs::read_to_string(MAIN_INPUT)?;
    let result_1 = solve_1(&input_string, (101, 103));
    println!("part_1: {result_1}");
    let result_2 = solve_2(&input_string, (101, 103));
    println!("part_2: {result_2}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input_string = fs::read_to_string(TEST_INPUT).unwrap();
        let result = solve_1(&input_string, (11, 7));
        assert_eq!(result, EXPECTED_TEST_RESULT_1);
    }
}
