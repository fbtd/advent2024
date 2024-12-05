#![allow(dead_code)]
#![allow(unused_imports)]

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while_m_n},
    character::complete::{alpha1, digit1, one_of},
    combinator::map_res,
    error::{Error as NomError, ErrorKind},
    multi::{many0, many1},
    sequence::tuple,
    Err, IResult,
};

use std::collections::HashMap;
use std::error::Error;

const TOTAL_BOXES: usize = 256;

#[derive(Debug, PartialEq)]
enum Action {
    Remove,
    Add(u32),
}

#[derive(Debug, PartialEq)]
struct Instruction<'a> {
    label: &'a str,
    action: Action,
}

impl Instruction<'_> {
    fn hash(&self) -> u8 {
        hash(self.label)
    }
}

fn hash(id: &str) -> u8 {
    let mut sum: u32 = 0;
    for c in id.chars() {
        sum += c as u32;
        sum *= 17;
        sum %= 256;
    }
    sum as u8
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction, NomError<&str>> {
    let (input, label) = alpha1(input)?;
    let (mut input, sep) = alt((tag("="), tag("-")))(input)?;
    let action = match sep {
        "=" => {
            let (new_input, focal_length_str) = digit1(input)?;
            input = new_input;
            let focal_length: u32 = focal_length_str
                .parse()
                .map_err(|_| Err::Failure(NomError::new(input, ErrorKind::Digit)))?;
            Action::Add(focal_length)
        }
        "-" => Action::Remove,
        _ => panic!("you should not be here"),
    };
    (input, _) = one_of(",\n")(input)?;

    Ok((input, Instruction { label, action }))
}

pub fn solve(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut boxes: [Vec<(&str, u32)>; TOTAL_BOXES] = core::array::from_fn(|_| Vec::new());

    let (_, instructions) = many1(parse_instruction)(input).unwrap();
    for instruction in instructions {
        let mut lens_index: Option<usize> = None;
        for (i, lens) in boxes[instruction.hash() as usize].iter().enumerate() {
            if lens.0 == instruction.label {
                lens_index = Some(i);
                break;
            }
        }
        match (&instruction.action, lens_index) {
            (Action::Add(focal_len), Some(index)) => {
                boxes[instruction.hash() as usize][index].1 = *focal_len
            }
            (Action::Add(focal_len), None) => {
                boxes[instruction.hash() as usize].push((instruction.label, *focal_len))
            }
            (Action::Remove, Some(index)) => _ = boxes[instruction.hash() as usize].remove(index),
            (Action::Remove, None) => (),
        }
    }

    let mut total = 0;
    for (i, boxx) in boxes.iter().enumerate() {
        for (j, lens) in boxx.iter().enumerate() {
            total += (i + 1) as u32 * (j + 1) as u32 * lens.1;
        }
    }
    Ok(total as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("rn"), 0);
        assert_eq!(hash("qp"), 1);
    }

    #[test]
    fn test_parse_instruction() {
        let test_input = "rn=1,cm-,qp=3\n";
        let expected_instruction = Instruction {
            label: "rn",
            action: Action::Add(1),
        };
        assert_eq!(
            parse_instruction(test_input),
            Ok(("cm-,qp=3\n", expected_instruction))
        );
    }

    #[test]
    fn test_parse_instruction_last() {
        let test_input = "rn=1\n";
        let expected_instruction = Instruction {
            label: "rn",
            action: Action::Add(1),
        };
        assert_eq!(
            parse_instruction(test_input),
            Ok(("", expected_instruction))
        );
    }
}
