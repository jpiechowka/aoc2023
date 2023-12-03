use rayon::prelude::*;

use crate::parser::parse_games;

mod parser;

fn main() {
    let input_file = include_str!("../input_p2.txt");
    let solution = part2(input_file);
    println!("{solution}");
}

fn part2(input: &str) -> u32 {
    let games = parse_games(input).expect("should parse games");
    games
        .1
        .par_iter()
        .map(|game| {
            let mut red_required: u32 = 0;
            let mut green_required: u32 = 0;
            let mut blue_required: u32 = 0;

            game.rounds.iter().for_each(|round| {
                round.iter().for_each(|cube| match cube.color {
                    "red" => {
                        if cube.count > red_required {
                            red_required = cube.count;
                        }
                    }
                    "green" => {
                        if cube.count > green_required {
                            green_required = cube.count;
                        }
                    }
                    "blue" => {
                        if cube.count > blue_required {
                            blue_required = cube.count;
                        }
                    }
                    _ => {}
                })
            });

            red_required * green_required * blue_required
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

        assert_eq!(part2(input), 2286);
    }
}
