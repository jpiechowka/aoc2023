use std::time::Instant;

use crate::parser::parse_races_part1;

mod parser;

fn main() {
    let start_time = Instant::now();
    let input_file = include_str!("../input_p1.txt");
    let solution = part1(input_file);
    let execution_time = start_time.elapsed().as_secs_f64();
    println!("[{execution_time:?} seconds] {solution}");
}

fn part1(input: &str) -> u64 {
    let (_, races) = parse_races_part1(input).expect("should parse input");

    races
        .iter()
        .map(|race| {
            (1..race.time)
                .filter(|speed| {
                    let distance = speed * (race.time - speed);
                    distance > race.distance
                })
                .count() as u64
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d06p1_full_input() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(part1(input), 288);
    }
}
