use std::error::Error;

struct PuzzleIter<'a> {
    puzzle: &'a Puzzle,
    position: usize
    direction: (i32, i32),
    first_call: bool,
}
impl<'a> PuzzleIter<'a> {
    fn new(puzzle: &'a Puzzle, start: usize, direction: (i32, i32)) -> Self {
        PuzzleIter {
            puzzle,
            position: start,
            direction,
            first_call: true,
        }
    }
}
impl<'a> Iterator for PuzzleIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_call {
            self.first_call = false;
            self.puzzle.letters.get(self.position).copied()
        } else {
            let p = self.position as i32
                + self.direction.0
                + self.direction.1 * self.puzzle.width as i32;
            let x = self.position as i32 % self.puzzle.width as i32 + self.direction.0;
            let y = self.position as i32 / self.puzzle.height as i32 + self.direction.1;
            if p < 0
                || p >= (self.puzzle.width * self.puzzle.height) as i32
                || x >= self.puzzle.width as i32
                || x < 0
                || y >= self.puzzle.height as i32
                || y < 0
            {
                None
            } else {
                self.position = p as usize;
                self.puzzle.letters.get(self.position).copied()
            }
        }
    }
}

struct Puzzle {
    letters: Vec<char>,
    width: usize,
    height: usize,
}

impl Puzzle {
    fn new(input_str: &str) -> Self {
        let height: usize = input_str.lines().count();
        let width: usize = input_str.lines().nth(0).unwrap().chars().count();
        let letters: Vec<char> = input_str.chars().filter(|c| *c != '\n').collect();
        Puzzle {
            letters,
            width,
            height,
        }
    }

    fn iter_letters(&self, start: usize, direction: (i32, i32)) -> PuzzleIter {
        PuzzleIter::new(&self, start, direction)
    }
}

pub fn solve(input: &str) -> Result<i32, Box<dyn Error>> {
    let puzzle = Puzzle::new(input);
    let directions: [(i32, i32); 8] = [
        (1, 0),
        (1, 1),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 0),
        (-1, 1),
        (-1, -1),
    ];
    let target = vec!['X', 'M', 'A', 'S'];
    let mut matches = 0;
    for i in 0..puzzle.width * puzzle.height {
        for (x, y) in directions {
            let found: Vec<char> = puzzle.iter_letters(i, (x, y)).take(4).collect();
            if found == target {
                matches += 1;
            }
        }
    }
    Ok(matches)
}

const TEST_STRING: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let puzzle = Puzzle::new(TEST_STRING);
        assert_eq!(puzzle.width, 10);
        assert_eq!(puzzle.height, 10);
        assert_eq!(puzzle.letters.iter().count(), 100);
    }

}
