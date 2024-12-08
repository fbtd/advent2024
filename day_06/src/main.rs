#![allow(unused_imports)]
#![allow(dead_code)]

use std::error::Error;
use std::fmt;
use std::fs;

const MAIN_INPUT: &str = "input.txt";
const TEST_INPUT: &str = "test_input.txt";
const EXPECTED_TEST_RESULT_1: i32 = 41;
const EXPECTED_TEST_RESULT_2: i32 = 6;

struct Lab {
    tiles: Vec<char>,
    width: usize,
    height: usize,
    guard: usize,
}

impl Lab {
    fn new(s: &str) -> Self {
        let width = s.find('\n').expect("newline not found");
        let mut guard = s.find('^').expect("guard not found");
        guard -= guard / (width + 1);
        let tiles: Vec<char> = s.chars().filter(|c| *c != '\n').collect();
        let height = tiles.len() / width;
        Lab {
            tiles,
            width,
            height,
            guard,
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

    fn is_loop(&self, g_dir: usize) -> bool {
        let mut tiles = self.tiles.clone();
        let mut guard = self.guard;
        let mut g_dir = g_dir;
        tiles[guard] = char::from_u32(g_dir as u32 + 0x30).expect("invalid direction");
        while let Some(tile_index) = self.tile_index(guard, DIRECTIONS[g_dir]) {
            if tiles[tile_index] == '#' {
                g_dir += 1;
                g_dir %= 4;
            } else if tiles[tile_index]
                == char::from_u32(g_dir as u32 + 0x30).expect("invalid direction")
            {
                return true;
            } else {
                tiles[tile_index] = char::from_u32(g_dir as u32 + 0x30).expect("invalid direction");
                guard = tile_index;
            }
        }
        false
    }
}
const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn solve_1(input_str: &str) -> i32 {
    let mut lab = Lab::new(input_str);
    let mut g_dir: usize = 3;
    lab.tiles[lab.guard] = 'X';
    while let Some(tile_index) = lab.tile_index(lab.guard, DIRECTIONS[g_dir]) {
        if lab.tiles[tile_index] == '#' {
            g_dir += 1;
            g_dir %= 4;
        } else {
            lab.tiles[tile_index] = 'X';
            lab.guard = tile_index;
        }
    }
    lab.tiles.iter().filter(|c| **c == 'X').count() as i32
}

fn solve_2(input_str: &str) -> i32 {
    let mut lab = Lab::new(input_str);
    let original_guard = lab.guard;
    let mut g_dir: usize = 3;

    // X-ify the path
    lab.tiles[lab.guard] = 'X';
    while let Some(tile_index) = lab.tile_index(lab.guard, DIRECTIONS[g_dir]) {
        if lab.tiles[tile_index] == '#' {
            g_dir += 1;
            g_dir %= 4;
        } else {
            lab.tiles[tile_index] = 'X';
            lab.guard = tile_index;
        }
    }

    let mut total_loops = 0;
    lab.guard = original_guard;
    for i in 0..lab.width * lab.height {
        if lab.tiles[i] == 'X' {
            lab.tiles[i] = '#';
            if lab.is_loop(3) {total_loops +=1 }
            lab.tiles[i] = 'X';
        }
    }
    total_loops
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
    fn test_is_loop() {
        let input_string = fs::read_to_string(TEST_INPUT).unwrap();
        let mut lab = Lab::new(&input_string);
        assert!(!lab.is_loop(3));

        lab.tiles[63] = '#';
        assert!(lab.is_loop(3));
    }
}
