use itertools::{repeat_n, Itertools};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, line_ending, space1};
use nom::combinator::{all_consuming, map_res};
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

/// Represents the types of tiles in a puzzle input.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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
    pub arrangements: Vec<u8>,
}

impl PuzzleLine {
    /// Generates all available permutations of the `PuzzleLine` tiles.
    ///
    /// # Returns
    /// A vector of `PuzzleLine` objects representing all available permutations.
    pub fn generate_available_permutations(&self) -> Vec<Self> {
        let unknown_tiles_count = self
            .tiles
            .iter()
            .filter(|t| t == &&TileType::Unknown)
            .count();

        let possible_tile_options = vec![TileType::Operational, TileType::Damaged];

        let permutations = repeat_n(&possible_tile_options, unknown_tiles_count)
            .multi_cartesian_product()
            .map(|permutation| {
                let mut permutation = permutation.into_iter();
                let updated_tiles: Vec<TileType> = self
                    .tiles
                    .iter()
                    .map(|tile| {
                        if tile == &TileType::Unknown {
                            permutation
                                .next()
                                .cloned()
                                .unwrap_or_else(|| panic!("ran out of permutation values"))
                        } else {
                            *tile
                        }
                    })
                    .collect();

                PuzzleLine {
                    tiles: updated_tiles,
                    arrangements: self.arrangements.clone(),
                }
            })
            .collect();

        permutations
    }

    /// Checks if the arrangement of tiles is correct.
    ///
    /// The function iterates over the tiles in `self.tiles` and checks if they are arranged correctly. An arrangement is considered correct
    /// if it matches the `self.arrangements` vector.
    ///
    /// # Returns
    ///
    /// Returns `true` if the arrangement is correct, otherwise returns `false`.
    ///
    /// # Panics
    ///
    /// This function will panic if it encounters an unexpected unknown tile (`TileType::Unknown`).
    pub fn is_tiles_arrangement_correct(&self) -> bool {
        let mut current_arrangement = Vec::new();
        let mut current_value = 0;

        for tile in &self.tiles {
            match tile {
                TileType::Damaged => current_value += 1,
                TileType::Operational => {
                    if current_value > 0 {
                        current_arrangement.push(current_value);
                        current_value = 0;
                    }
                }
                TileType::Unknown => panic!("encountered unexpected unknown tile"),
            }
        }

        // Push if there was a sequence of Damaged or Operational tiles at the end
        if current_value > 0 {
            current_arrangement.push(current_value);
        }

        current_arrangement == self.arrangements
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
        separated_list1(tag(","), parse_u8),
    )(input_line)?;

    Ok((
        input,
        PuzzleLine {
            tiles: puzzle_line.0,
            arrangements: puzzle_line.1,
        },
    ))
}

/// Parses a string slice (`&str`) into an unsigned 8-bit integer (`u8`).
///
/// This function takes an input string slice and attempts to parse it into
/// an `u8`. It uses the `digit1` parser to extract one or more decimal digits
/// from the input. Then it uses `str::parse` to convert the extracted string
/// slice into an `u8` value. If parsing is successful, it returns the parsed
/// value along with the remaining input string slice. Otherwise, it returns an
/// error indicating the parsing failure.
///
/// # Arguments
///
/// * `input` - The input string slice to be parsed.
///
/// # Return Value
///
/// If parsing is successful, it returns a `nom::IResult<&str, u8>` which is an
/// `enum` with two variants:
///
/// * `Ok((remaining, value))` - Represents a successful parsing, where `remaining`
///   is the remaining input string slice and `value` is the parsed `u8` value.
///
/// * `Err(nom::Err)` - Represents a parsing failure, where `nom::Err` is an `enum`
///   with various variants indicating different types of parsing errors.
fn parse_u8(input: &str) -> IResult<&str, u8> {
    map_res(digit1, str::parse::<u8>)(input)
}
