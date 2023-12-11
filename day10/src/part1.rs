use std::iter::successors;
use std::time::Instant;

use glam::IVec2;
use nom_locate::LocatedSpan;

use crate::parser::{parse_input, Direction, PipeType};

mod parser;

fn main() {
    let start_time = Instant::now();
    let input_file = include_str!("../input_p1.txt");
    let solution = part1(input_file);
    let execution_time = start_time.elapsed().as_secs_f64();
    println!("[{execution_time:?} seconds] {solution}");
}

fn part1(input: &str) -> u32 {
    let (_input, pipes) = parse_input(LocatedSpan::new(input)).expect("should parse input");

    let start_pos = pipes
        .iter()
        .find_map(|(k, v)| (v == &PipeType::StartingPosition).then_some(k))
        .expect("should find starting position");

    let north = *start_pos + IVec2::new(0, -1);
    let north_pos = pipes
        .get(&north)
        .is_some_and(|pipe_type| {
            matches!(
                pipe_type,
                PipeType::NorthSouthVerticalPipe
                    | PipeType::SouthEastBend
                    | PipeType::SouthWestBend
            )
        })
        .then_some((Direction::South, north));

    let south = *start_pos + IVec2::new(0, 1);
    let south_pos = pipes
        .get(&south)
        .is_some_and(|pipe_type| {
            matches!(
                pipe_type,
                PipeType::NorthSouthVerticalPipe
                    | PipeType::NorthEastBend
                    | PipeType::NorthWestBend
            )
        })
        .then_some((Direction::North, south));

    let east = *start_pos + IVec2::new(1, 0);
    let east_pos = pipes
        .get(&east)
        .is_some_and(|pipe_type| {
            matches!(
                pipe_type,
                PipeType::EastWestHorizontalPipe
                    | PipeType::NorthWestBend
                    | PipeType::SouthWestBend
            )
        })
        .then_some((Direction::West, east));

    let west = *start_pos + IVec2::new(-1, 0);
    let west_pos = pipes
        .get(&west)
        .is_some_and(|pipe_type| {
            matches!(
                pipe_type,
                PipeType::EastWestHorizontalPipe
                    | PipeType::NorthEastBend
                    | PipeType::SouthEastBend
            )
        })
        .then_some((Direction::East, west));

    let mut iters = vec![north_pos, south_pos, east_pos, west_pos]
        .into_iter()
        .flatten()
        .map(|tuple| {
            successors(Some(tuple), |(coming_from_direction, current_pos)| {
                let pipe_type = pipes
                    .get(current_pos)
                    .expect("should not ask for non-existent position");

                let next_direction = match (coming_from_direction, pipe_type) {
                    (Direction::North, PipeType::NorthSouthVerticalPipe) => Direction::South,
                    (Direction::North, PipeType::NorthEastBend) => Direction::East,
                    (Direction::North, PipeType::NorthWestBend) => Direction::West,
                    (Direction::South, PipeType::NorthSouthVerticalPipe) => Direction::North,
                    (Direction::South, PipeType::SouthEastBend) => Direction::East,
                    (Direction::South, PipeType::SouthWestBend) => Direction::West,
                    (Direction::East, PipeType::EastWestHorizontalPipe) => Direction::West,
                    (Direction::East, PipeType::NorthEastBend) => Direction::North,
                    (Direction::East, PipeType::SouthEastBend) => Direction::South,
                    (Direction::West, PipeType::EastWestHorizontalPipe) => Direction::East,
                    (Direction::West, PipeType::NorthWestBend) => Direction::North,
                    (Direction::West, PipeType::SouthWestBend) => Direction::South,
                    _ => panic!("unable to get next direction"),
                };

                Some(match next_direction {
                    Direction::North => (Direction::South, *current_pos + IVec2::new(0, -1)),
                    Direction::South => (Direction::North, *current_pos + IVec2::new(0, 1)),
                    Direction::East => (Direction::West, *current_pos + IVec2::new(1, 0)),
                    Direction::West => (Direction::East, *current_pos + IVec2::new(-1, 0)),
                })
            })
        });

    let path_a = iters.next().expect("path A should exist");
    let path_b = iters.next().expect("path B should exist");

    let final_pos = path_a
        .zip(path_b)
        .position(|(a, b)| a.1 == b.1)
        .expect("should meet in the middle");

    final_pos as u32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d10p1_input1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        assert_eq!(part1(input), 4);
    }

    #[test]
    fn d10p1_input2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        assert_eq!(part1(input), 8);
    }
}
