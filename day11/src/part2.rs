use std::collections::BTreeSet;
use std::time::Instant;

use glam::I64Vec2;
use itertools::Itertools;

use crate::parser::{parse_input, DataType};

mod parser;

fn main() {
    let start_time = Instant::now();
    let input_file = include_str!("../input_p1.txt");
    let solution = part2(input_file, 1_000_000);
    let execution_time = start_time.elapsed().as_secs_f64();
    println!("[{execution_time:?} seconds] {solution}");
}

fn part2(input: &str, expansion_size: u64) -> i64 {
    let galaxy_map = parse_input(input);
    let galaxy_coordinates = get_galaxies_coordinates(&galaxy_map);

    let row_indexes_to_expand = galaxy_map
        .iter()
        .enumerate()
        .filter_map(|(row_idx, vec_of_rows)| {
            vec_of_rows
                .iter()
                .all(|data_type| *data_type == DataType::EmptySpace)
                .then_some(row_idx)
        })
        .collect::<BTreeSet<usize>>();

    let initial_row_len = galaxy_map
        .first()
        .map(|v| v.len())
        .expect("should get value of line_len");

    let mut column_indexes_to_expand: BTreeSet<usize> = BTreeSet::new();
    for col_idx in 0..initial_row_len {
        if galaxy_map.iter().all(|vec_of_rows| {
            vec_of_rows
                .get(col_idx)
                .expect("should get element from vector")
                == &DataType::EmptySpace
        }) {
            column_indexes_to_expand.insert(col_idx);
        }
    }

    let expanded_galaxy_coordinates = galaxy_coordinates
        .iter()
        .map(|galaxy_coord| {
            let rows_added = row_indexes_to_expand
                .iter()
                .filter(|row| row < &&(galaxy_coord.y as usize))
                .count() as i64
                * (expansion_size as i64 - 1);
            let cols_added = column_indexes_to_expand
                .iter()
                .filter(|col| col < &&(galaxy_coord.x as usize))
                .count() as i64
                * (expansion_size as i64 - 1);

            I64Vec2::new(galaxy_coord.x + cols_added, galaxy_coord.y + rows_added)
        })
        .collect::<Vec<I64Vec2>>();

    expanded_galaxy_coordinates
        .iter()
        .combinations(2)
        .map(|combinations_vec| galaxy_manhattan_distance(combinations_vec[0], combinations_vec[1]))
        .sum()
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
/// A vector of I64Vec2 objects, each representing the coordinates of a galaxy.
fn get_galaxies_coordinates(expanded_galaxy_map: &Vec<Vec<DataType>>) -> Vec<I64Vec2> {
    let mut galaxies_coordinates: Vec<I64Vec2> = Vec::new();

    expanded_galaxy_map
        .iter()
        .enumerate()
        .for_each(|(row_idx, line)| {
            line.iter().enumerate().for_each(|(col_idx, data_type)| {
                if *data_type == DataType::Galaxy {
                    galaxies_coordinates.push(I64Vec2::new(col_idx as i64, row_idx as i64));
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
/// use im::vector::I64Vec2;
///
/// let g1 = I64Vec2::new(1, 2);
/// let g2 = I64Vec2::new(3, 4);
/// let distance = galaxy_manhattan_distance(&g1, &g2);
/// assert_eq!(distance, 4);
/// ```
fn galaxy_manhattan_distance(g1: &I64Vec2, g2: &I64Vec2) -> i64 {
    (g2.x - g1.x).abs() + (g2.y - g1.y).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d11p2_full_input() {
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

        assert_eq!(part2(input, 10), 1030);
        assert_eq!(part2(input, 100), 8410);
    }
}
