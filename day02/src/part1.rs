use std::ops::Not;

use rayon::prelude::*;

use crate::parser::parse_games;

mod parser;

const RED_MAX_COUNT: u32 = 12;
const GREEN_MAX_COUNT: u32 = 13;
const BLUE_MAX_COUNT: u32 = 14;

fn main() {
    let input_file = include_str!("../input_p1.txt");
    let solution = part1(input_file);
    println!("{solution}");
}

fn part1(input: &str) -> u32 {
    let games = parse_games(input).expect("should parse games");
    games
        .1
        .par_iter()
        .filter_map(|game| {
            game.rounds
                .iter()
                .any(|round| {
                    round.iter().any(|cube| {
                        // Game is always valid, unless there is round where number of cubes is grater than max counts
                        cube.color == "red" && cube.count > RED_MAX_COUNT
                            || cube.color == "green" && cube.count > GREEN_MAX_COUNT
                            || cube.color == "blue" && cube.count > BLUE_MAX_COUNT
                    })
                })
                .not()
                .then_some(game.id)
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d01p1_full_input() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part1(input), 8);
    }
}
