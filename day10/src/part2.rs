use std::collections::HashSet;
use std::iter::successors;
use std::time::Instant;

use glam::IVec2;
use nom_locate::LocatedSpan;

use crate::parser::{parse_input, Direction, PipeType};

mod parser;

#[derive(Debug, Eq, PartialEq)]
enum Status {
    In,
    Out,
}

fn main() {
    let start_time = Instant::now();
    let input_file = include_str!("../input_p2.txt");
    let solution = part2(input_file);
    let execution_time = start_time.elapsed().as_secs_f64();
    println!("[{execution_time:?} seconds] {solution}");
}

fn part2(input: &str) -> u32 {
    let (_input, pipes) = parse_input(LocatedSpan::new(input), true).expect("should parse input");

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
    let zipped = path_a.zip(path_b);
    let mut pipe_locations: HashSet<IVec2> = HashSet::from([*start_pos]);

    for (path_a_node, path_b_node) in zipped {
        pipe_locations.insert(path_a_node.1);
        pipe_locations.insert(path_b_node.1);

        if path_a_node.1 == path_b_node.1 {
            break;
        }
    }

    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let mut status = Status::Out;

            line.chars()
                .enumerate()
                .filter(|(x, _)| {
                    let pos = IVec2::new(*x as i32, y as i32);
                    let pipe_type = pipes.get(&pos).expect("should get a valid pipe");

                    if pipe_locations.contains(&pos) {
                        if [
                            PipeType::StartingPosition,
                            PipeType::NorthSouthVerticalPipe,
                            PipeType::SouthWestBend,
                            PipeType::SouthEastBend,
                        ]
                        .contains(pipe_type)
                        {
                            status = match status {
                                Status::In => Status::Out,
                                Status::Out => Status::In,
                            };
                        };
                        false
                    } else {
                        match status {
                            Status::In => true,
                            Status::Out => false,
                        }
                    }
                })
                .count() as u32
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d10p2_input1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        assert_eq!(part2(input), 4);
    }

    #[test]
    fn d10p2_input2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        assert_eq!(part2(input), 8);
    }

    #[test]
    fn d10p2_input3() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        assert_eq!(part2(input), 10);
    }
}
