use std::time::Instant;

use itertools::Itertools;

use crate::parser::parse_report;

mod parser;

fn main() {
    let start_time = Instant::now();
    let input_file = include_str!("../input_p1.txt");
    let solution = part1(input_file);
    let execution_time = start_time.elapsed().as_secs_f64();
    println!("[{execution_time:?} seconds] {solution}");
}

fn part1(input: &str) -> i64 {
    let report = parse_report(input);
    report.iter().map(extrapolate_values).sum()
}

fn extrapolate_values(history: &Vec<i64>) -> i64 {
    let mut differences = calculate_differences(history);
    let mut difference_to_add: i64 = 0;

    while !differences.iter().all(|&x| x == 0) {
        difference_to_add += differences.last().unwrap();
        differences = calculate_differences(&differences);
    }

    history.last().unwrap() + difference_to_add
}

fn calculate_differences(input: &[i64]) -> Vec<i64> {
    input
        .iter()
        .tuple_windows()
        .map(|(first, second)| second - first)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d09p1_full_input() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(part1(input), 114);
    }
}
