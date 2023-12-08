use std::collections::BTreeMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, line_ending, multispace1};
use nom::combinator::eof;
use nom::multi::{fold_many1, many1};
use nom::sequence::{delimited, separated_pair, terminated};
use nom::{IResult, Parser};

#[derive(Debug)]
pub enum Instruction {
    Left,
    Right,
}

pub fn parse_input(input: &str) -> IResult<&str, (Vec<Instruction>, BTreeMap<&str, (&str, &str)>)> {
    let (input, instructions) = many1(alt((
        complete::char('R').map(|_| Instruction::Right),
        complete::char('L').map(|_| Instruction::Left),
    )))(input)?;
    let (input, _) = multispace1(input)?;

    let (input, map) = fold_many1(
        terminated(
            separated_pair(
                alpha1,
                tag(" = "),
                delimited(
                    complete::char('('),
                    separated_pair(alpha1, tag(", "), alpha1),
                    complete::char(')'),
                ),
            ),
            alt((line_ending, eof)),
        ),
        BTreeMap::new,
        |mut acc: BTreeMap<&str, (&str, &str)>, (key, value)| {
            acc.insert(key, value);
            acc
        },
    )(input)?;

    Ok((input, (instructions, map)))
}
