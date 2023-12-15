/// Represents the type of data in the universe.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DataType {
    Galaxy,
    EmptySpace,
}

/// Parses input string into a two-dimensional vector of DataType.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Returns
///
/// Returns a two-dimensional vector of DataType where each inner vector represents a line in the input string.
///
/// # Example
///
/// ```
/// use crate::DataType;
///
/// let input = ".#.\n.#.\n";
/// let parsed_data = parse_input(input);
/// assert_eq!(parsed_data[0][0], DataType::EmptySpace);
/// assert_eq!(parsed_data[0][1], DataType::Galaxy);
/// assert_eq!(parsed_data[1][0], DataType::EmptySpace);
/// assert_eq!(parsed_data[1][1], DataType::Galaxy);
/// ```
pub fn parse_input(input: &str) -> Vec<Vec<DataType>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => DataType::EmptySpace,
                    '#' => DataType::Galaxy,
                    val => panic!("unable to parse unknown data type: {}", val),
                })
                .collect()
        })
        .collect()
}
