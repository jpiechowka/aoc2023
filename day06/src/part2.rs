use std::time::Instant;

use crate::parser::parse_race_part2;

mod parser;

fn main() {
    let start_time = Instant::now();
    let input_file = include_str!("../input_p2.txt");
    let solution = part2(input_file);
    let execution_time = start_time.elapsed().as_secs_f64();
    println!("[{execution_time:?} seconds] {solution}");
}

fn part2(input: &str) -> u64 {
    let (_, race) = parse_race_part2(input).expect("should parse input");

    (1..race.time)
        .filter(|speed| {
            let distance = speed * (race.time - speed);
            distance > race.distance
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d06p2_full_input() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(part2(input), 71503);
    }
}
