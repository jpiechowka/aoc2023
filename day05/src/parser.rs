use std::ops::Range;

use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{digit1, line_ending, multispace1, space1};
use nom::combinator::map_res;
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, tuple};
use nom::IResult;

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<u64>,
    pub seed_to_soil: Vec<(Range<u64>, Range<u64>)>,
    pub soil_to_fertilizer: Vec<(Range<u64>, Range<u64>)>,
    pub fertilizer_to_water: Vec<(Range<u64>, Range<u64>)>,
    pub water_to_light: Vec<(Range<u64>, Range<u64>)>,
    pub light_to_temperature: Vec<(Range<u64>, Range<u64>)>,
    pub temperature_to_humidity: Vec<(Range<u64>, Range<u64>)>,
    pub humidity_to_location: Vec<(Range<u64>, Range<u64>)>,
}

pub fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, _) = multispace1(input)?;
    let (input, seed_to_soil) = parse_map(input, "seed-to-soil")?;
    let (input, _) = multispace1(input)?;
    let (input, soil_to_fertilizer) = parse_map(input, "soil-to-fertilizer")?;
    let (input, _) = multispace1(input)?;
    let (input, fertilizer_to_water) = parse_map(input, "fertilizer-to-water")?;
    let (input, _) = multispace1(input)?;
    let (input, water_to_light) = parse_map(input, "water-to-light")?;
    let (input, _) = multispace1(input)?;
    let (input, light_to_temperature) = parse_map(input, "light-to-temperature")?;
    let (input, _) = multispace1(input)?;
    let (input, temperature_to_humidity) = parse_map(input, "temperature-to-humidity")?;
    let (input, _) = multispace1(input)?;
    let (input, humidity_to_location) = parse_map(input, "humidity-to-location")?;

    Ok((
        input,
        Almanac {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        },
    ))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag_no_case("seeds: "), separated_list1(space1, parse_u64))(input)
}

fn parse_map<'a>(
    input: &'a str,
    map_name: &'a str,
) -> IResult<&'a str, Vec<(Range<u64>, Range<u64>)>> {
    preceded(
        tuple((tag(map_name), tag_no_case(" map:"))),
        many1(preceded(line_ending, line)),
    )(input)
}

fn line(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (destination, source, num)) = tuple((
        parse_u64,
        preceded(tag(" "), parse_u64),
        preceded(tag(" "), parse_u64),
    ))(input)?;

    Ok((
        input,
        (source..(source + num), destination..(destination + num)),
    ))
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse::<u64>)(input)
}
