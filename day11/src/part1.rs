use std::collections::BTreeSet;
use std::time::Instant;

use glam::IVec2;
use itertools::Itertools;

use crate::parser::{parse_input, DataType};

mod parser;

fn main() {
    let start_time = Instant::now();
    let input_file = include_str!("../input_p1.txt");
    let solution = part1(input_file);
    let execution_time = start_time.elapsed().as_secs_f64();
    println!("[{execution_time:?} seconds] {solution}");
}

fn part1(input: &str) -> i32 {
    let galaxy_map = parse_input(input);
    let expanded_galaxy_map = expand_galaxy_map(galaxy_map);
    let galaxies_coordinates = get_galaxies_coordinates(expanded_galaxy_map);

    galaxies_coordinates
        .iter()
        .tuple_combinations()
        .map(|(g1, g2)| galaxy_manhattan_distance(g1, g2))
        .sum()
}

/// Expands the galaxy map by inserting empty rows and columns.
///
/// # Arguments
///
/// * `input` - A vector of vectors representing the original galaxy map.
///
/// # Returns
///
/// A new vector of vectors with empty rows and columns inserted.
fn expand_galaxy_map(input: Vec<Vec<DataType>>) -> Vec<Vec<DataType>> {
    let mut expanded_galaxy_map = input.clone();
    let mut row_indexes_to_expand: BTreeSet<usize> = BTreeSet::new();
    let mut column_indexes_to_expand: BTreeSet<usize> = BTreeSet::new();

    let initial_row_len = input
        .first()
        .map(|v| v.len())
        .expect("should get value of line_len");

    // get indexes of rows to expand
    input.iter().enumerate().for_each(|(row_idx, vec_of_rows)| {
        if vec_of_rows
            .iter()
            .all(|data_type| *data_type == DataType::EmptySpace)
        {
            row_indexes_to_expand.insert(row_idx);
        }
    });

    // and next expand rows in the expanded_galaxy_map, taking into account already inserted rows
    row_indexes_to_expand
        .iter()
        .enumerate()
        .for_each(|(map_idx, row_idx)| {
            expanded_galaxy_map.insert(
                *row_idx + map_idx,
                vec![DataType::EmptySpace; initial_row_len],
            )
        });

    // get indexes of columns to expand
    for col_idx in 0..initial_row_len {
        if expanded_galaxy_map.iter().all(|vec_of_rows| {
            vec_of_rows
                .get(col_idx)
                .expect("should get element from vector")
                == &DataType::EmptySpace
        }) {
            column_indexes_to_expand.insert(col_idx);
        }
    }

    // and next expand columns in the expanded_galaxy_map, taking into account already inserted columns
    column_indexes_to_expand
        .iter()
        .enumerate()
        .for_each(|(map_idx, col_idx)| {
            expanded_galaxy_map.iter_mut().for_each(|row| {
                row.insert(*col_idx + map_idx, DataType::EmptySpace);
            });
        });

    expanded_galaxy_map
}

/// Retrieves the coordinates of all galaxies in the provided expanded galaxy map.
///
/// # Arguments
///
/// * `expanded_galaxy_map` - A 2-dimensional vector representing the expanded galaxy map,
///                           where each position contains a DataType enum value.
///
/// # Returns
///
/// A vector of IVec2 objects, each representing the coordinates of a galaxy.
fn get_galaxies_coordinates(expanded_galaxy_map: Vec<Vec<DataType>>) -> Vec<IVec2> {
    let mut galaxies_coordinates: Vec<IVec2> = Vec::new();

    expanded_galaxy_map
        .iter()
        .enumerate()
        .for_each(|(row_idx, line)| {
            line.iter().enumerate().for_each(|(col_idx, data_type)| {
                if *data_type == DataType::Galaxy {
                    galaxies_coordinates.push(IVec2::new(col_idx as i32, row_idx as i32));
                }
            })
        });

    galaxies_coordinates
}

/// Calculates the Manhattan distance between two galaxies.
///
/// # Arguments
///
/// * `g1` - The first galaxy.
/// * `g2` - The second galaxy.
///
/// # Returns
///
/// The Manhattan distance between the two galaxies.
///
/// # Example
///
/// ```
/// use im::vector::IVec2;
///
/// let g1 = IVec2::new(1, 2);
/// let g2 = IVec2::new(3, 4);
/// let distance = galaxy_manhattan_distance(&g1, &g2);
/// assert_eq!(distance, 4);
/// ```
fn galaxy_manhattan_distance(g1: &IVec2, g2: &IVec2) -> i32 {
    (g2.x - g1.x).abs() + (g2.y - g1.y).abs()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(IVec2::new(0, 0), IVec2::new(0, 1), 1)]
    #[case(IVec2::new(0, 0), IVec2::new(1, 0), 1)]
    #[case(IVec2::new(0, 0), IVec2::new(1, 1), 2)]
    #[case(IVec2::new(1, 2), IVec2::new(3, 4), 4)]
    #[case(IVec2::new(0, 11), IVec2::new(5, 11), 5)]
    #[case(IVec2::new(1, 6), IVec2::new(5, 11), 9)]
    #[case(IVec2::new(4, 0), IVec2::new(9, 10), 15)]
    #[case(IVec2::new(0, 2), IVec2::new(12, 7), 17)]
    fn d11p1_distance_test(#[case] g1: IVec2, #[case] g2: IVec2, #[case] expected_distance: i32) {
        assert_eq!(galaxy_manhattan_distance(&g1, &g2), expected_distance);
    }

    #[test]
    fn d11p1_combinations_test() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let galaxy_map = parse_input(input);
        let expanded_galaxy_map = expand_galaxy_map(galaxy_map);
        let galaxies_coordinates = get_galaxies_coordinates(expanded_galaxy_map);
        let combinations_num = galaxies_coordinates
            .iter()
            .tuple_combinations::<(&IVec2, &IVec2)>()
            .try_len();
        assert_eq!(combinations_num, Ok(36));
    }

    #[test]
    fn d11p1_galaxy_expansion_test() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let expected_expanded_map = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

        let expanded_map = expand_galaxy_map(parse_input(input));
        let expanded_map_parsed = expanded_map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|data_type| match data_type {
                        DataType::Galaxy => '#',
                        DataType::EmptySpace => '.',
                    })
                    .join("")
            })
            .join("\n");

        assert_eq!(expanded_map_parsed, expected_expanded_map);
    }

    #[test]
    fn d11p1_full_input() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(part1(input), 374);
    }
}
