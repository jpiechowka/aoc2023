use std::collections::HashMap;

use glam::IVec2;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::multispace0;
use nom::combinator::all_consuming;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::{IResult, Parser};
use nom_locate::LocatedSpan;

#[derive(Debug, Eq, PartialEq)]
pub enum PipeType {
    NorthSouthVerticalPipe,
    // |
    EastWestHorizontalPipe,
    // -
    NorthEastBend,
    // L
    NorthWestBend,
    // J
    SouthWestBend,
    // 7
    SouthEastBend,
    // F
    Ground,
    // .
    StartingPosition, // S
}

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
pub struct PipeInfo<'a> {
    pub span: LocatedSpan<&'a str, IVec2>,
    pub pipe_type: PipeType,
}

pub fn parse_input(
    input: LocatedSpan<&str>,
    adjust_for_part2: bool,
) -> IResult<LocatedSpan<&str>, HashMap<IVec2, PipeType>> {
    let (input, pipes) = all_consuming(many1(terminated(
        alt((
            tag("|").map(with_location).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::NorthSouthVerticalPipe,
            }),
            tag("-").map(with_location).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::EastWestHorizontalPipe,
            }),
            tag_no_case("L").map(with_location).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::NorthEastBend,
            }),
            tag_no_case("J").map(with_location).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::NorthWestBend,
            }),
            tag("7").map(with_location).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::SouthWestBend,
            }),
            tag_no_case("F").map(with_location).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::SouthEastBend,
            }),
            tag_no_case("S").map(with_location).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::StartingPosition,
            }),
            tag(".").map(with_location).map(|span| PipeInfo {
                span,
                pipe_type: PipeType::Ground,
            }),
        )),
        multispace0,
    )))(input)?;

    if !adjust_for_part2 {
        Ok((
            input,
            pipes
                .into_iter()
                .filter_map(|pipe_info| {
                    (pipe_info.pipe_type != PipeType::Ground)
                        .then_some((pipe_info.span.extra, pipe_info.pipe_type))
                })
                .collect(),
        ))
    } else {
        Ok((
            input,
            pipes
                .into_iter()
                .map(|pipe_info| (pipe_info.span.extra, pipe_info.pipe_type))
                .collect(),
        ))
    }
}

fn with_location(span: LocatedSpan<&str>) -> LocatedSpan<&str, IVec2> {
    let x = span.get_column() as i32 - 1;
    let y = span.location_line() as i32 - 1;
    span.map_extra(|_| IVec2::new(x, y))
}
