use std::iter::zip;

use nom::bytes::complete::tag_no_case;
use nom::character::complete::{digit1, multispace1, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use nom::IResult;

#[derive(Debug)]
pub struct Race {
    pub time: u64,
    pub distance: u64,
}

pub fn parse_races_part1(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times) = preceded(
        tuple((tag_no_case("Time:"), space1)),
        separated_list1(space1, parse_u64),
    )(input)?;

    let (input, distances) = preceded(
        tuple((multispace1, tag_no_case("Distance:"), space1)),
        separated_list1(space1, parse_u64),
    )(input)?;

    let races = zip(times, distances)
        .map(|race| Race {
            time: race.0,
            distance: race.1,
        })
        .collect();

    Ok((input, races))
}

pub fn parse_race_part2(input: &str) -> IResult<&str, Race> {
    let (input, times) = preceded(
        tuple((tag_no_case("Time:"), space1)),
        separated_list1(space1, digit1),
    )(input)?;

    let (input, distances) = preceded(
        tuple((multispace1, tag_no_case("Distance:"), space1)),
        separated_list1(space1, digit1),
    )(input)?;

    let concat_time: String = times.into_iter().collect();
    let concat_distance: String = distances.into_iter().collect();

    let race = Race {
        time: concat_time.parse().expect("should parse concatenated time"),
        distance: concat_distance
            .parse()
            .expect("should parse concatenated distance"),
    };

    Ok((input, race))
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse::<u64>)(input)
}
