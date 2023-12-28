use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, line_ending, space1};
use nom::combinator::{all_consuming, map_res};
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

/// Represents the types of tiles in a puzzle input.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum TileType {
    Operational, //.
    Damaged,     // #
    Unknown,     // ?
}

/// A struct representing a line in a puzzle.
///
/// Each line consists of a vector of tiles and a vector of arrangements.
#[derive(Debug, Clone)]
pub struct PuzzleLine {
    pub tiles: Vec<TileType>,
    pub arrangements: Vec<usize>,
}

/// Calculates the count of valid arrangements of tiles with the given configurations and cache.
///
/// # Arguments
///
/// * `tiles` - A slice of `TileType` representing the tiles.
/// * `arrangements` - A slice of `usize` representing the configurations of the tiles.
/// * `cache` - A mutable `HashMap` storing the previously calculated count of arrangements.
///
/// # Returns
///
/// The count of valid arrangements based on the given tiles and configurations.
pub fn arrangements_count_with_cache(
    tiles: &[TileType],
    arrangements: &[usize],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(count) = cache.get(&(tiles.len(), arrangements.len())) {
        return *count;
    }

    let mut count = 0;

    if arrangements.is_empty() {
        count = if tiles.contains(&TileType::Damaged) {
            0
        } else {
            1
        };

        cache.insert((tiles.len(), arrangements.len()), count);
        return count;
    }

    for offset in 0..tiles.len() {
        if tiles[0..offset].contains(&TileType::Damaged) || offset + arrangements[0] > tiles.len() {
            break;
        }

        if tiles[offset..offset + arrangements[0]].contains(&TileType::Operational) {
            continue;
        }

        if arrangements.len() == 1 {
            if offset + arrangements[0] == tiles.len() {
                count += 1;
                break;
            } else {
                count +=
                    arrangements_count_with_cache(&tiles[offset + arrangements[0]..], &[], cache);
                continue;
            };
        } else if offset + arrangements[0] + 1 > tiles.len() {
            break;
        } else if tiles[offset + arrangements[0]] == TileType::Damaged {
            continue;
        }

        count += arrangements_count_with_cache(
            &tiles[offset + arrangements[0] + 1..],
            &arrangements[1..],
            cache,
        );
    }

    cache.insert((tiles.len(), arrangements.len()), count);

    count
}

impl PuzzleLine {
    /// Unfolds the records of the object by adding more copies of its tiles and arrangements.
    ///
    /// Each row in the tiles will have the list of spring tiles replaced with five copies of itself,
    /// separated by unknown tile type. Similarly, the list of arrangements will be replaced with
    /// five copies of itself.
    ///
    /// # Returns
    ///
    /// Returns a new object with unfolded tiles and arrangements.
    pub fn unfold_records(&self) -> Self {
        // To unfold the records, on each row, replace the list of spring tiles with five copies of itself
        // (separated by ?) and replace the list of contiguous groups of damaged springs with five copies of itself
        // (separated by ,)
        let mut unfolded_tiles = self.tiles.clone();
        let mut unfolded_arrangements = self.arrangements.clone();

        for _ in 0..4 {
            unfolded_tiles.push(TileType::Unknown);
            unfolded_tiles.extend(&self.tiles);
            unfolded_arrangements.extend(&self.arrangements);
        }

        Self {
            tiles: unfolded_tiles,
            arrangements: unfolded_arrangements,
        }
    }
}

impl Display for PuzzleLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let row_string_representation = self
            .tiles
            .iter()
            .map(|tile| match tile {
                TileType::Operational => ".",
                TileType::Damaged => "#",
                TileType::Unknown => "?",
            })
            .join("");

        let arrangements_string_representation = self.arrangements.iter().join(",");

        write!(
            f,
            "{} {}",
            row_string_representation, arrangements_string_representation
        )
    }
}

/// Parses the input string and returns a result representing
/// the parsing status and the list of puzzle lines.
///
/// # Arguments
///
/// * `input` - The input string to be parsed.
///
/// # Returns
///
/// Returns a `IResult` enum, representing the parsing status and the
/// list of puzzle lines. The `IResult` variant could be `Ok` if the
/// parsing is successful, or `Err` if there was an error while parsing.
pub fn parse_input(input: &str) -> IResult<&str, Vec<PuzzleLine>> {
    all_consuming(separated_list1(line_ending, parse_line))(input)
}

/// Parses a puzzle line from an input string.
///
/// The input string should be in the format "{tiles} {arrangements}", where:
/// - `{tiles}` is a sequence of characters representing tile types.
///   - '.' represents operational tiles.
///   - '#' represents damaged tiles.
///   - '?' represents unknown tiles.
/// - `{arrangements}` is a comma-separated list of unsigned integers, representing possible tile arrangements.
///
/// # Arguments
///
/// * `input_line` - The input string to parse.
///
/// # Returns
///
/// - `Ok((remaining_input, puzzle_line))` if the parsing is successful,
///    - `remaining_input` is the input string that is not parsed.
///    - `puzzle_line` is the parsed `PuzzleLine` struct.
/// - `Err(NomErr(error_kind))` if an error occurs during parsing.
pub fn parse_line(input_line: &str) -> IResult<&str, PuzzleLine> {
    // Example input: "???.### 1,1,3"
    let (input, puzzle_line) = separated_pair(
        many1(alt((
            char('.').map(|_| TileType::Operational),
            char('#').map(|_| TileType::Damaged),
            char('?').map(|_| TileType::Unknown),
        ))),
        space1,
        separated_list1(tag(","), parse_usize),
    )(input_line)?;

    Ok((
        input,
        PuzzleLine {
            tiles: puzzle_line.0,
            arrangements: puzzle_line.1,
        },
    ))
}

/// Parses a string slice (`&str`) into an usize (`usize`).
///
/// This function takes an input string slice and attempts to parse it into
/// an `usize`. It uses the `digit1` parser to extract one or more decimal digits
/// from the input. Then it uses `str::parse` to convert the extracted string
/// slice into an `usize` value. If parsing is successful, it returns the parsed
/// value along with the remaining input string slice. Otherwise, it returns an
/// error indicating the parsing failure.
///
/// # Arguments
///
/// * `input` - The input string slice to be parsed.
///
/// # Return Value
///
/// If parsing is successful, it returns a `nom::IResult<&str, usize>` which is an
/// `enum` with two variants:
///
/// * `Ok((remaining, value))` - Represents a successful parsing, where `remaining`
///   is the remaining input string slice and `value` is the parsed `usize` value.
///
/// * `Err(nom::Err)` - Represents a parsing failure, where `nom::Err` is an `enum`
///   with various variants indicating different types of parsing errors.
fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse::<usize>)(input)
}
