#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;

const MAIN_INPUT: &str = "input.txt";
const TEST_INPUT: &str = "test_input.txt";
const EXPECTED_TEST_RESULT_1: i32 = 36;
const EXPECTED_TEST_RESULT_2: i32 = 81;

struct Grid {
    tiles: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(s: &str) -> Self {
        let width = s.find('\n').expect("newline not found");
        let tiles: Vec<u8> = s
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        //let tiles: Vec<char> = s.chars().filter(|c| *c != '\n').collect();
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

    fn xy_to_tile(&self, xy: (i32, i32)) -> Option<usize> {
        if xy.0 >= self.width as i32 || xy.0 < 0 || xy.1 >= self.height as i32 || xy.1 < 0 {
            return None;
        }
        Some(xy.0 as usize + xy.1 as usize * self.width)
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
    //fn print(&self) {
    //    let mut s = String::new();
    //    for (i, t) in self.tiles.iter().enumerate() {
    //        if i % self.width == 0 {
    //            s.push('\n')
    //        }
    //        s.push(*t);
    //    }
    //    print!("{s}\n");
    //}
}
const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];


fn n_of_valid_paths(grid: &Grid, bottom: usize) -> usize {
    let mut destinations_reached: HashSet<usize> = HashSet::new();
    let mut valid_tiles: Vec<usize> = vec![bottom];
    while let Some(tile) = valid_tiles.pop() {
        let level = grid.tiles[tile];
        for direction in DIRECTIONS {
            if let Some(next_tile) = grid.tile_index(tile, direction) {
               //dbg!(tile, next_tile, level, "");
                if level == 8 && grid.tiles[next_tile] == 9 {
                    destinations_reached.insert(next_tile);
                } else if grid.tiles[next_tile] == level + 1 {
                    valid_tiles.push(next_tile);
                }
            }
        }
    }
    destinations_reached.len()
}
fn rating(grid: &Grid, bottom: usize) -> usize {
    let mut destinations_reached: usize=0;
    let mut valid_tiles: Vec<usize> = vec![bottom];
    while let Some(tile) = valid_tiles.pop() {
        let level = grid.tiles[tile];
        for direction in DIRECTIONS {
            if let Some(next_tile) = grid.tile_index(tile, direction) {
               //dbg!(tile, next_tile, level, "");
                if level == 8 && grid.tiles[next_tile] == 9 {
                    destinations_reached += 1;
                } else if grid.tiles[next_tile] == level + 1 {
                    valid_tiles.push(next_tile);
                }
            }
        }
    }
    destinations_reached
}

fn solve_1(input_str: &str) -> i32 {
    let grid = Grid::new(input_str);
    (0..grid.width * grid.height)
        .filter(|i| grid.tiles[*i] == 0)
        .map(|i| n_of_valid_paths(&grid, i) as i32)
        .sum()
}

fn solve_2(input_str: &str) -> i32 {
    let grid = Grid::new(input_str);
    (0..grid.width * grid.height)
        .filter(|i| grid.tiles[*i] == 0)
        .map(|i| rating(&grid, i) as i32)
        .sum()
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

    #[test]
    fn test_n_of_valid_paths() {
        let one = Grid::new(
            "0123
1234
8765
9876",
        );
        assert_eq!(n_of_valid_paths(&one, 0), 1);

        let two = Grid::new(
            "0000000
0001000
0002000
6543456
7004007
8005678
9000009",
        );
        assert_eq!(n_of_valid_paths(&two, 3), 2);
    }

    #[test]
    fn test_n_of_valid_paths_buggy() {
        let one = Grid::new(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        );
        assert_eq!(n_of_valid_paths(&one, 42), 1);
    }
}
