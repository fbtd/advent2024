#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

mod plane;
use plane::Plane;

mod grid;
use grid::Grid;

const MAIN_INPUT: &str = "input.txt";
const TEST_INPUT_SMALL: &str = "test_input_small.txt";
const TEST_INPUT: &str = "test_input.txt";
const TEST_INPUT_3: &str = "test_input_3.txt";
const EXPECTED_TEST_RESULT_1: i32 = 2028;
const EXPECTED_TEST_RESULT_2: i32 = 10092;
const EXPECTED_TEST_RESULT_3: i32 = 9021;

fn move_boxes(grid: &mut Grid, start: usize, direction: (i32, i32)) -> bool {
    let mut t = start;
    loop {
        if let Some(new_t) = grid.tile_index(t, direction) {
            match grid.tiles[new_t] {
                '.' => {
                    grid.tiles[new_t] = 'O';
                    grid.tiles[start] = '.';
                    return true;
                }
                '#' => return false,
                'O' => t = new_t,
                t => panic!("invalid tile: {}", t),
            }
        } else {
            return false;
        }
    }
}

fn move_robot(grid: &mut Grid, robot: usize, direction: (i32, i32)) -> usize {
    let target_tile: usize = grid.tile_index(robot, direction).unwrap();
    match grid.tiles[target_tile] {
        '.' => return target_tile,
        '#' => return robot,
        'O' => {
            if move_boxes(grid, target_tile, direction) {
                return target_tile;
            } else {
                return robot;
            }
        }
        t => panic!("invalid tile: {}", t),
    }
}

fn solve_1(input_string: &str) -> i32 {
    let char_to_dir: HashMap<char, (i32, i32)> =
        HashMap::from([('<', (-1, 0)), ('>', (1, 0)), ('^', (0, -1)), ('v', (0, 1))]);
    let mut parts = input_string.split("\n\n").into_iter();
    let part_1 = parts.next().unwrap();
    let mut grid = Grid::new(part_1);
    let mut robot = part_1.find('@').unwrap();
    robot -= robot / (grid.width + 1);
    grid.tiles[robot] = '.';
    let moves: String = parts.next().unwrap().lines().collect();
    for m in moves.chars() {
        robot = move_robot(&mut grid, robot, *char_to_dir.get(&m).unwrap());
    }

    let mut sum = 0;
    for i in 0..grid.width * grid.height {
        if grid.tiles[i] == 'O' {
            sum += (i % grid.width) + (i / grid.width) * 100;
        }
    }
    sum as i32
}

type Jail = Box<[Cell<char>]>;

fn move_robot_2(jail: &Jail, plane: &Plane, robot: usize, direction: (i32, i32)) -> usize {
    let target_tile: usize = plane.tile_plus_xy(robot, direction).unwrap();
    match jail[target_tile].get() {
        '.' => return target_tile,
        '#' => return robot,
        '[' | ']' => {
            if push_boxes(jail, plane, robot, direction) {
                return target_tile;
            } else {
                return robot;
            }
        }
        t => panic!("invalid tile: {}", t),
    }
}

fn push_boxes(jail: &Jail, plane: &Plane, robot: usize, direction: (i32, i32)) -> bool {
    if direction.1 == 0 {
        // horizontal movement
        let mut t = robot;
        while let Some(new_t) = plane.tile_plus_xy(t, direction) {
            match jail[new_t].get() {
                '.' => {
                    let mut tt = new_t;
                    while tt != robot {
                        let next_t = (tt as i32 - direction.0) as usize;
                        jail[tt].set(jail[next_t].get());
                        tt = next_t;
                    }
                    return true;
                }
                '#' => return false,
                ']' | '[' => t = new_t,
                c => panic!("invalid tile: {}", c),
            }
        }
    }

    // vertical movement
    let mut to_check: Vec<usize> = vec![plane.tile_plus_xy(robot, direction).unwrap()];
    let mut to_move: Vec<usize> = Vec::new();
    let mut visited: Vec<usize> = Vec::new();

    // what needs to be moved
    while let Some(check_me) = to_check.pop() {
        visited.push(check_me);
        let c = jail[check_me].get();
        if c == '[' || c == ']' {
            to_move.push(check_me);
            if let Some(t) = plane.tile_plus_xy(check_me, direction) {
                if !visited.contains(&t) {
                    to_check.push(t);
                }
            }
            if c == '[' && !visited.contains(&(check_me + 1)) {
                to_check.push(check_me + 1);
            } else if c == ']' && !visited.contains(&(check_me - 1)) {
                to_check.push(check_me - 1);
            }
        }
    }

    // is there space to move it?
    for t in to_move.iter() {
        if let Some(new_t) = plane.tile_plus_xy(*t, direction) {
            if jail[new_t].get() == '#' {
                return false;
            }
        } else {
            return false;
        }
    }

    to_move.sort();
    to_move.dedup();
    // move the boxes
    if direction.1 == 1 {
        to_move.reverse();
    }
    //dbg!(&to_move);
    for t in to_move {
        let next_t = plane.tile_plus_xy(t, direction).unwrap();
        jail[next_t].set(jail[t].get());
        jail[t].set('.');
    }
    true
}

fn print_jail(jail: &Jail, plane: &Plane, robot: usize) {
    let mut s = String::new();
    for (i, t) in jail.iter().enumerate() {
        if i % plane.width == 0 {
            s.push('\n')
        }
        if i == robot {
            s.push('@')
        } else {
            s.push(t.get());
        }
    }
    print!("{s}\n");
}

use std::cell::Cell;
fn jail_from_str(input: &str) -> Jail {
    let v: Vec<Cell<char>> = input
        .chars()
        .filter(|c| *c != '\n')
        .map(Cell::new)
        .collect();
    v.into_boxed_slice()
}

fn solve_2(input_string: &str) -> i32 {
    let char_to_dir: HashMap<char, (i32, i32)> =
        HashMap::from([('<', (-1, 0)), ('>', (1, 0)), ('^', (0, -1)), ('v', (0, 1))]);
    let mut parts = input_string.split("\n\n").into_iter();
    let part_1 = parts.next().unwrap();
    let part_1 = &part_1.replace("#", "##");
    let part_1 = &part_1.replace("O", "[]");
    let part_1 = &part_1.replace(".", "..");
    let part_1 = &part_1.replace("@", "@.");
    let mut robot = part_1.find('@').unwrap();
    let width = part_1.find('\n').unwrap();
    let jail = jail_from_str(part_1);
    let height = jail.len() / width;
    robot -= robot / (width + 1);
    jail[robot].set('.');
    let plane = Plane::new(width, height);
    let moves: String = parts.next().unwrap().lines().collect();

    for c in moves.chars() {
        robot = move_robot_2(&jail, &plane, robot, char_to_dir[&c]);
    }

    print_jail(&jail, &plane, robot);
    let mut sum = 0;
    for i in 0..plane.len() {
        if jail[i].get() == '[' {
            sum += (i % plane.width) + (i / plane.width) * 100;
        }
    }
    sum as i32
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_string = fs::read_to_string(MAIN_INPUT)?;
    let result_1 = solve_1(&input_string);
    println!("part_1: {result_1}");
    //let input_string = fs::read_to_string(TEST_INPUT_3)?;
    let result_2 = solve_2(&input_string);
    println!("part_2: {result_2}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_small() {
        let input_string = fs::read_to_string(TEST_INPUT_SMALL).unwrap();
        let result = solve_1(&input_string);
        assert_eq!(result, EXPECTED_TEST_RESULT_1);
    }

    #[test]
    fn test_part_1() {
        let input_string = fs::read_to_string(TEST_INPUT).unwrap();
        let result = solve_1(&input_string);
        assert_eq!(result, EXPECTED_TEST_RESULT_2);
    }

    #[test]
    fn test_part_2() {
        let input_string = fs::read_to_string(TEST_INPUT).unwrap();
        let result = solve_2(&input_string);
        assert_eq!(result, EXPECTED_TEST_RESULT_3);
    }

    #[test]
    fn test_jail_from_str() {
        let input_string = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########";
        let jail = jail_from_str(&input_string);
        assert_eq!(jail.len(), 64);
        assert_eq!(jail[0].get(), '#');
        assert_eq!(jail[9].get(), '.');
    }
}
