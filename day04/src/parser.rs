use std::collections::HashSet;

use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete;
use nom::character::complete::{digit1, line_ending, space0, space1};
use nom::combinator::map_res;
use nom::multi::{fold_many1, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated, tuple};
use nom::{IResult, Parser};

#[derive(Debug)]
pub struct Card {
    pub id: u32,
    pub winning_numbers: HashSet<u32>,
    pub numbers_to_check: HashSet<u32>,
}

// Example input: Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
pub fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, card)(input)
}

fn card(input: &str) -> IResult<&str, Card> {
    // let (input, card_id) = preceded(tag_no_case("card "), map_res(digit1, str::parse::<u32>))(input)?;
    let (input, card_id) = delimited(
        tuple((tag_no_case("card"), space1)),
        map_res(digit1, str::parse::<u32>),
        tuple((tag(":"), space1)),
    )(input)?;

    separated_pair(numbers, tuple((tag("|"), space1)), numbers)
        .map(|(winning_numbers, numbers_to_check)| Card {
            id: card_id,
            winning_numbers,
            numbers_to_check,
        })
        .parse(input)
}

fn numbers(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        HashSet::new,
        |mut acc: HashSet<u32>, num| {
            acc.insert(num);
            acc
        },
    )(input)
}
