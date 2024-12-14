#![allow(unused_imports)]
#![allow(dead_code)]

use regex::Regex;

use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

const MAIN_INPUT: &str = "input.txt";
const TEST_INPUT: &str = "test_input.txt";
const EXPECTED_TEST_RESULT_1: i32 = 480;
const EXPECTED_TEST_RESULT_2: i32 = 55312;

const TRY_RANGE: u64 = 1_000;

#[derive(Debug)]
struct Game {
    ax: u64,
    ay: u64,
    bx: u64,
    by: u64,
    tx: u64,
    ty: u64,
}

impl Game {
    fn new(input_string: &str) -> Game {
        let re = Regex::new(r"X.(\d+), Y.(\d+)").unwrap();
        let mut lines_iter = input_string.lines();

        let line = lines_iter.next().expect("invalid game lines");
        let cap = re.captures(line).unwrap();
        let ax = cap[1].parse::<u64>().unwrap();
        let ay = cap[2].parse::<u64>().unwrap();

        let line = lines_iter.next().expect("invalid game lines");
        let cap = re.captures(line).unwrap();
        let bx = cap[1].parse::<u64>().unwrap();
        let by = cap[2].parse::<u64>().unwrap();

        let line = lines_iter.next().expect("invalid game lines");
        let cap = re.captures(line).unwrap();
        let tx = cap[1].parse::<u64>().unwrap();
        let ty = cap[2].parse::<u64>().unwrap();
        Game {
            ax,
            ay,
            bx,
            by,
            tx,
            ty,
        }
    }

    fn solve(&self, bias: u64) -> u64 {
        let mut solutions: Vec<u64> = Vec::new();

        let tx = self.tx + bias;
        let ty = self.ty + bias;
        let max_a = min(tx / self.ax, ty / self.ay);
        let max_b = min(tx / self.bx, ty / self.by);

        for ia in 0..=max_a {
            for ib in 0..=max_b {
                if ia * self.ax + ib * self.bx == tx && ia * self.ay + ib * self.by == ty {
                    return ia * 3 + ib;
                }
            }
        }
        0
        //dbg!(&solutions.len());
        //let solution = solutions.into_iter().min().unwrap_or(0u64);
        //solution
    }

    fn solve_far(&self, bias: u64) -> u64 {
        let ax = self.ax as f64;
        let ay = self.ay as f64;
        let bx = self.bx as f64;
        let by = self.by as f64;
        let tx = self.tx + bias;
        let ty = self.ty + bias;
        let multi_b: f64 = (ax - ay) / (by - bx);
        if multi_b < 0.0 {
            return 0;
        }

        if (ty as f64 / by) % 1.0 == 0.0 && (ty / self.by) * self.bx == tx {
            dbg!(bx, by, multi_b, tx, ty);
        }

        let multi = bias as f64 / (ax + bx * multi_b);
        let start_a = multi as u64;
        let start_b = (multi * multi_b) as u64;

        for ia in start_a - TRY_RANGE..start_a + TRY_RANGE {
            for ib in start_b - TRY_RANGE..start_b + TRY_RANGE {
                if ia * self.ax + ib * self.bx == tx && ia * self.ay + ib * self.by == ty {
                    return ia * 3 + ib;
                }
            }
        }

        //dbg!(
        //    multi_b,
        //    multi,
        //    start_a * self.ax + start_b * self.bx,
        //    start_a * self.ay + start_b * self.by,
        //    tx,
        //    ty
        //);
        0
    }
}

fn solve_1(input_string: &str) -> i32 {
    let games: Vec<Game> = input_string.split("\n\n").map(Game::new).collect();
    games.iter().map(|g| g.solve(0)).sum::<u64>() as i32
}

fn solve_2(input_string: &str) -> u64 {
    let games: Vec<Game> = input_string.split("\n\n").map(Game::new).collect();
    games
        .iter()
        .map(|g| g.solve_far(10000000000000u64))
        .sum::<u64>()
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
}
