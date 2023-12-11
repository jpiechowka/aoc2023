use std::time::Instant;

use crate::parser::parse_input;

mod parser;

fn main() {
    let start_time = Instant::now();
    let input_file = include_str!("../input_p2.txt");
    let solution = part2(input_file);
    let execution_time = start_time.elapsed().as_secs_f64();
    println!("[{execution_time:?} seconds] {solution}");
}

fn part2(input: &str) -> i64 {
    let pipes = parse_input(input);
    dbg!(&pipes);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d10p2_full_input() {
        let input = "";

        assert_eq!(part2(input), 0);
    }
}