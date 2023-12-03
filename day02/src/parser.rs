use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete;
use nom::character::complete::{alpha1, digit1, line_ending};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

#[derive(Debug)]
pub struct Cube<'a> {
    pub color: &'a str,
    pub count: u32,
}

#[derive(Debug)]
pub struct Game<'a> {
    pub id: u32,
    pub rounds: Vec<Vec<Cube<'a>>>,
}

pub fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}

// Example input: Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag_no_case("game "), map_res(digit1, str::parse))(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round))(input)?;
    Ok((input, Game { rounds, id }))
}

// Example input: 3 blue, 4 red
fn round(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;
    Ok((input, cubes))
}

// Example input: 4 red
fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (count, color)) = separated_pair(complete::u32, tag(" "), alpha1)(input)?;
    Ok((input, Cube { color, count }))
}
