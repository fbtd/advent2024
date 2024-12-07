use std::error::Error;

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

    fn get_x(&self, origin: usize) -> Option<Vec<char>> {
        let x_shape: Vec<(i32, i32)> = vec![(-1, -1), (-1, 1), (0, 0), (1, 1), (1, -1)];

        if origin < self.width
            || origin >= self.width * (self.height - 1)
            || origin % self.width == 0
            || origin % self.width == self.width - 1
        {
            return None;
        }

        let r: Vec<char> = x_shape
            .iter()
            .map(|delta| {
                let p: i32 = origin as i32 + delta.0 + delta.1 * self.width as i32;
                self.letters[p as usize]
            })
            .collect();
        Some(r)
    }
}

pub fn is_mas(cross: Vec<char>) -> bool {
    let valids = [
        vec!['S', 'M', 'A', 'M', 'S'],
        vec!['M', 'M', 'A', 'S', 'S'],
        vec!['S', 'S', 'A', 'M', 'M'],
        vec!['M', 'S', 'A', 'S', 'M'],
    ];
    for valid in valids {
        if valid == cross {
            return true
        }
    }
    false
}

pub fn solve(input: &str) -> Result<i32, Box<dyn Error>> {
    let puzzle = Puzzle::new(input);
    let result:usize = (0..puzzle.width * puzzle.height)
        .into_iter()
        .map(|i| puzzle.get_x(i as usize))
        .filter_map(|o| o)
        .map(|x| is_mas(x))
        .filter(|x| *x)
        .count();
    Ok(result as i32)
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

    #[test]
    fn test_x() {
        let puzzle = Puzzle::new(TEST_STRING);
        assert_eq!(puzzle.get_x(1), None);
        assert_eq!(puzzle.get_x(20), None);
        assert_eq!(puzzle.get_x(29), None);
        assert_eq!(puzzle.get_x(93), None);
        assert_eq!(
            puzzle.get_x(12).map(|mut v| {
                v.sort();
                v
            }),
            Some(vec!['A', 'M', 'M', 'S', 'S'])
        );
    }
    #[test]
    fn test_is_mas() {
        assert!(is_mas(vec!['M', 'M', 'A', 'S', 'S']));
        assert!(is_mas(vec!['S', 'M', 'A', 'M', 'S']));
        assert_eq!(is_mas(vec!['S', 'S', 'A', 'M', 'S']), false);
    }
}
