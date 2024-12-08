#![allow(unused_imports)]
#![allow(dead_code)]

use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;

const MAIN_INPUT: &str = "input.txt";
const TEST_INPUT: &str = "test_input.txt";
const EXPECTED_TEST_RESULT_1: i32 = 14;
const EXPECTED_TEST_RESULT_2: i32 = 34;

struct Grid {
    tiles: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(s: &str) -> Self {
        let width = s.find('\n').expect("newline not found");
        let tiles: Vec<char> = s.chars().filter(|c| *c != '\n').collect();
        let height = tiles.len() / width;
        Grid {
            tiles,
            width,
            height,
        }
    }

    fn tile_index(&self, origin: usize, dirs: (i32, i32)) -> Option<usize> {
        let x = dirs.0;
        let y = dirs.1;
        let new_x: i32 = (origin % self.width) as i32 + x;
        let new_y: i32 = (origin / self.width) as i32 + y;
        if new_x >= self.width as i32 || new_x < 0 || new_y >= self.height as i32 || new_y < 0 {
            return None;
        }
        Some(new_x as usize + new_y as usize * self.width)
    }

    fn tile_to_xy(&self, index: usize) -> (i32, i32) {
        let x = index as i32 % self.width as i32;
        let y = index as i32 / self.width as i32;
        (x, y)
    }

    fn delta_xy(&self, indexes: (usize, usize)) -> (i32, i32) {
        let first = self.tile_to_xy(indexes.0);
        let second = self.tile_to_xy(indexes.1);
        (second.0 - first.0, second.1 - first.1)
    }

    // FIXME: implement fmt
    fn print(&self) {
        let mut s = String::new();
        for (i,t) in self.tiles.iter().enumerate() {
            if i % self.width == 0 {s.push('\n')}
            s.push(*t);
        }
        print!("{s}\n");
    }
}
const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn solve_1(input_str: &str) -> i32 {
    let mut grid = Grid::new(input_str);
    let mut antennas: HashMap<char, Vec<usize>> = HashMap::new();

    for (i, c) in grid.tiles.iter().enumerate() {
        if *c == '.' {
            continue;
        }
        antennas.entry(*c).or_insert(Vec::<usize>::new()).push(i);
    }

    for (_, locations) in antennas.iter() {
        for pair in locations.iter().combinations(2) {
            let pair = (*pair[0], *pair[1]);
            let delta = grid.delta_xy(pair);
            grid.tile_index(pair.1, delta).map(|i| grid.tiles[i] = '#');
            let delta = (-delta.0, -delta.1);
            grid.tile_index(pair.0, delta).map(|i| grid.tiles[i] = '#');
        }
    }
    grid.tiles.iter().filter(|c| **c == '#').count() as i32
}

fn solve_2(input_str: &str) -> i32 {
    let mut grid = Grid::new(input_str);
    let mut antennas: HashMap<char, Vec<usize>> = HashMap::new();

    for (i, c) in grid.tiles.iter().enumerate() {
        if *c == '.' {
            continue;
        }
        antennas.entry(*c).or_insert(Vec::<usize>::new()).push(i);
    }

    for (_, locations) in antennas.iter() {
        for pair in locations.iter().combinations(2) {
            let mut pair = (*pair[0], *pair[1]);
            let delta = grid.delta_xy(pair);
            let mut target_indexes: Vec<usize> = Vec::new();

            while let Some(index) = grid.tile_index(pair.1, delta) {
                target_indexes.push(index);
                pair.1 = index;
            }
            let delta = (-delta.0, -delta.1);
            while let Some(index) = grid.tile_index(pair.0, delta) {
                target_indexes.push(index);
                pair.0 = index;
            }

            for i in target_indexes {
                grid.tiles[i] = '#'
            }
        }
    }
    grid.tiles.iter().filter(|c| **c != '.').count() as i32
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
        assert_eq!(result, EXPECTED_TEST_RESULT_2);
    }
}
