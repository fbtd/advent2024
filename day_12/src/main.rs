#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

mod grid;
use grid::{Grid, DIRECTIONS};

const MAIN_INPUT: &str = "input.txt";
const TEST_INPUT: &str = "test_input.txt";
const EXPECTED_TEST_RESULT_1: i32 = 1930;
const EXPECTED_TEST_RESULT_2: i32 = 1206;

#[derive(Debug, PartialEq)]
struct Region {
    code: char,
    id: usize,
    perimeter: usize,
    area: usize,
    tiles: Vec<usize>,
}

impl Region {
    fn new(code: char) -> Region {
        Region {
            code,
            id: 0,
            perimeter: 0,
            area: 0,
            tiles: Vec::new(),
        }
    }
}

fn get_regiont(regions: &Vec<Region>, tile: usize, region_code: char) -> Option<&Region> {
    for region in regions.iter() {
        if region.tiles.contains(&tile) && region.code == region_code {
            return Some(region);
        }
    }
    None
}
fn get_region_mut(
    regions: &mut Vec<Region>,
    tile: usize,
    region_code: char,
) -> Option<&mut Region> {
    for region in regions.iter_mut() {
        if region.tiles.contains(&tile) && region.code == region_code {
            return Some(region);
        }
    }
    None
}

fn solve_1(input_str: &str) -> i32 {
    let grid = Grid::new(&input_str);

    let mut regions: Vec<Region> = Vec::new();
    let mut tiles_to_check: Vec<usize> = (0..grid.len()).collect();

    while tiles_to_check.len() > 0 {
        let tile = tiles_to_check[0];
        let code = grid.tiles[tile];
        let mut potential_neighbors: Vec<usize> = vec![tile];
        let mut region: Region = Region::new(code);

        while let Some(potential_neighbor) = potential_neighbors.pop() {
            if let Ok(i) = tiles_to_check.binary_search(&potential_neighbor) {
                tiles_to_check.remove(i);
                region.tiles.push(potential_neighbor);
                region.area += 1;
                for direction in DIRECTIONS {
                    if let Some(neighbor) = grid.tile_index(potential_neighbor, direction) {
                        if grid.tiles[neighbor] == code {
                            potential_neighbors.push(neighbor);
                        } else {
                            region.perimeter += 1;
                        }
                    } else {
                        region.perimeter += 1;
                    }
                }
            }
        }
        regions.push(region);
    }
    regions.iter().map(|v| v.area * v.perimeter).sum::<usize>() as i32
}

#[derive(PartialEq)]
enum Action {
    Same,
    Inside,
    Outside,
}

fn turn(grid: &Grid, tiles: &[usize], tile: usize, side: usize) -> (Action, usize, usize) {
    let next = grid.tile_index(tile, DIRECTIONS[(side + 1) % 4]);
    if next.is_some() && tiles.contains(&next.unwrap()) {
        let opposing = grid.tile_index(next.unwrap(), DIRECTIONS[side]);
        if opposing.is_some() && tiles.contains(&opposing.unwrap()) {
            (Action::Outside, opposing.unwrap(), (side + 3) % 4)
        } else {
            (Action::Same, next.unwrap(), side)
        }
    } else {
        (Action::Inside, tile, (side + 1) % 4)
    }
}

fn single_side_counter(grid: &Grid, tiles: &[usize]) -> usize {
    let mut tiles: Vec<usize> = tiles.iter().copied().collect();
    tiles.sort();
    let mut total_sides = 0;
    let first_tile = tiles[0];
    let first_side = 3;
    let mut is_first = true;

    let mut current_tile = first_tile;
    let mut current_side = first_side;

    loop {
        let t = turn(&grid, &tiles, current_tile, current_side);
        if t.0 != Action::Same {
            total_sides += 1;
        } else if is_first {
            //total_sides += 1;
        }
        is_first = false;
        current_tile = t.1;
        current_side = t.2;

        if current_tile == first_tile && current_side == first_side {
            break;
        }
    }
    if total_sides < 4 {
        panic!("invalid side number");
    }
    total_sides
}

fn side_counter(grid: &Grid, tiles: &[usize]) -> usize {
    let tiles: Vec<usize> = tiles.iter().copied().collect();
    let mut total_sides: usize = 0;

    let min_height :usize = tiles[0] / grid.width;
    let min_width :usize = tiles.iter().map(|t| t % grid.width).min().unwrap() % grid.height;
    let max_height :usize = *tiles.iter().last().unwrap() / grid.width;
    let max_width :usize = tiles.iter().map(|t| t % grid.width).max().unwrap() % grid.height;
    //dbg!();
    // check horizontallllly
    for side in [1, 3] {
        for y in min_height..max_height {
            let mut in_fence = false;
            for x in min_width..max_width {
                let t = x + y * grid.height;
                let opposing = grid.tile_index(t, DIRECTIONS[side]);
                if tiles.contains(&t) && (opposing.is_none() || !tiles.contains(&opposing.unwrap()))
                {
                    in_fence = true;
                } else if in_fence {
                    in_fence = false;
                    //dbg!((side, x, y));
                    total_sides += 1;
                }
            }
            if in_fence {
                //dbg!(("F", side, y));
                total_sides += 1;
            }
        }
    }

    // check verticalllllly
    for side in [0, 2] {
        for x in min_width..max_width {
            let mut in_fence = false;
            for y in min_height..max_height {
                let t = x + y * grid.height;
                let opposing = grid.tile_index(t, DIRECTIONS[side]);
                if tiles.contains(&t) && (opposing.is_none() || !tiles.contains(&opposing.unwrap()))
                {
                    in_fence = true;
                } else if in_fence {
                    in_fence = false;
                    total_sides += 1;
                }
            }
            if in_fence {
                total_sides += 1;
            }
        }
    }

    total_sides
}

fn solve_2(input_str: &str) -> i32 {
    let grid = Grid::new(&input_str);

    let mut regions: Vec<Region> = Vec::new();
    let mut tiles_to_check: Vec<usize> = (0..grid.len()).collect();

    while tiles_to_check.len() > 0 {
        let tile = tiles_to_check[0];
        let code = grid.tiles[tile];
        let mut potential_neighbors: Vec<usize> = vec![tile];
        let mut region: Region = Region::new(code);

        while let Some(potential_neighbor) = potential_neighbors.pop() {
            if let Ok(i) = tiles_to_check.binary_search(&potential_neighbor) {
                tiles_to_check.remove(i);
                region.tiles.push(potential_neighbor);
                region.area += 1;
                for direction in DIRECTIONS {
                    if let Some(neighbor) = grid.tile_index(potential_neighbor, direction) {
                        if grid.tiles[neighbor] == code {
                            potential_neighbors.push(neighbor);
                        } else {
                            region.perimeter += 1;
                        }
                    } else {
                        region.perimeter += 1;
                    }
                }
            }
        }
        regions.push(region);
    }

    regions
        .iter()
        .map(|v| v.area * side_counter(&grid, &v.tiles))
        .sum::<usize>() as i32
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

    //    #[test]
    //    fn test_part_2_abba() {
    //        let input_string = "AAAAAA
    //AAABBA
    //AAABBA
    //ABBAAA
    //ABBAAA
    //AAAAAA";
    //        let result = solve_2(&input_string);
    //        assert_eq!(result, EXPECTED_TEST_RESULT_2);
    //    }

    #[test]
    fn test_single() {
        let input_str = fs::read_to_string(TEST_INPUT).unwrap();
        let grid = Grid::new(&input_str);
        assert_eq!(side_counter(&grid, &vec![0]), 4);
        assert_eq!(side_counter(&grid, &vec![9]), 4);
        assert_eq!(side_counter(&grid, &vec![10]), 4);
        assert_eq!(side_counter(&grid, &vec![90]), 4);
        assert_eq!(side_counter(&grid, &vec![91]), 4);
        assert_eq!(side_counter(&grid, &vec![99]), 4);
    }

    #[test]
    fn test_shapes_sides() {
        let input_str = fs::read_to_string(TEST_INPUT).unwrap();
        let grid = Grid::new(&input_str);
        assert_eq!(side_counter(&grid, &vec![0, 1]), 4);
        assert_eq!(side_counter(&grid, &vec![10, 20, 30]), 4);
        assert_eq!(side_counter(&grid, &vec![10, 20, 30]), 4);
        assert_eq!(side_counter(&grid, &vec![0, 1, 10, 20, 30]), 6);
        assert_eq!(side_counter(&grid, &vec![0, 1, 10, 11, 20, 21, 30, 31]), 4);
        assert_eq!(side_counter(&grid, &vec![0, 1, 2, 10, 12]), 8);
        assert_eq!(side_counter(&grid, &vec![0, 1, 2, 10, 12, 20, 22]), 8);
    }

    #[test]
    fn test_shapes_cross() {
        let input_str = fs::read_to_string(TEST_INPUT).unwrap();
        let grid = Grid::new(&input_str);
        assert_eq!(side_counter(&grid, &vec![11, 20, 21, 22, 31]), 12);
    }

    #[test]
    fn test_shapes_spiral() {
        let input_str = fs::read_to_string(TEST_INPUT).unwrap();
        let grid = Grid::new(&input_str);
        assert_eq!(
            side_counter(&grid, &vec![0, 2, 3, 10, 13, 20, 21, 22, 23]),
            10
        );
        assert_eq!(
            side_counter(
                &grid,
                &vec![0, 1, 2, 3, 10, 20, 22, 23, 30, 33, 40, 41, 42, 43]
            ),
            12
        );
    }

    #[test]
    fn test_shapes_hole() {
        let input_str = fs::read_to_string(TEST_INPUT).unwrap();
        let grid = Grid::new(&input_str);
        assert_eq!(side_counter(&grid, &vec![0, 1, 2, 10, 12, 20, 21, 22]), 8);
    }
}
