use std::ops::Range;

use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{digit1, multispace1, space1};
use nom::combinator::map_res;
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, separated_pair, tuple};
use nom::{IResult, Parser};

#[derive(Debug)]
pub struct Mapping {
    pub mappings: Vec<Ranges>,
}

#[derive(Debug)]
pub struct Ranges {
    pub src: Range<u64>,
    pub dst: Range<u64>,
}

impl Mapping {
    pub fn translate(&self, value: u64) -> u64 {
        let valid_mapping = self
            .mappings
            .iter()
            .find(|mapping| mapping.src.contains(&value));

        let Some(mapping) = valid_mapping else {
            return value;
        };

        mapping.dst.start + (value - mapping.src.start)
    }
}

pub fn parse_almanac_part1(input: &str) -> IResult<&str, (Vec<u64>, Vec<Mapping>)> {
    let (input, seeds) = parse_seeds_part1(input)?;
    let (input, mappings) = many1(parse_maps)(input)?;

    Ok((input, (seeds, mappings)))
}

pub fn parse_almanac_part2(input: &str) -> IResult<&str, (Vec<Range<u64>>, Vec<Mapping>)> {
    let (input, seeds) = parse_seeds_part2(input)?;
    let (input, mappings) = many1(parse_maps)(input)?;

    Ok((input, (seeds, mappings)))
}

fn parse_seeds_part1(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag_no_case("seeds: "), separated_list1(space1, parse_u64))(input)
}

fn parse_seeds_part2(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    preceded(
        tag_no_case("seeds: "),
        separated_list1(
            space1,
            separated_pair(parse_u64, tag(" "), parse_u64)
                .map(|(start, length)| start..start + length),
        ),
    )(input)
}

fn parse_maps(input: &str) -> IResult<&str, Mapping> {
    let (input, _) = multispace1(input)?;

    preceded(
        tuple((
            alt((
                // TODO: optimize, take_until?
                tag_no_case("seed-to-soil"),
                tag_no_case("soil-to-fertilizer"),
                tag_no_case("fertilizer-to-water"),
                tag_no_case("water-to-light"),
                tag_no_case("light-to-temperature"),
                tag_no_case("temperature-to-humidity"),
                tag_no_case("humidity-to-location"),
            )),
            tag_no_case(" map:"),
        )),
        many1(preceded(multispace1, line)).map(|mappings| Mapping { mappings }),
    )(input)
}

fn line(input: &str) -> IResult<&str, Ranges> {
    let (input, (destination, source, length)) = tuple((
        parse_u64,
        preceded(tag(" "), parse_u64),
        preceded(tag(" "), parse_u64),
    ))(input)?;

    Ok((
        input,
        Ranges {
            src: source..source + length,
            dst: destination..destination + length,
        },
    ))
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse::<u64>)(input)
}
