use std::time::Instant;

use itertools::Itertools;

use crate::parser::parse_report;

mod parser;

fn main() {
    let start_time = Instant::now();
    let input_file = include_str!("../input_p2.txt");
    let solution = part2(input_file);
    let execution_time = start_time.elapsed().as_secs_f64();
    println!("[{execution_time:?} seconds] {solution}");
}

fn part2(input: &str) -> i64 {
    let report = parse_report(input);
    report.iter().map(extrapolate_previous_values).sum()
}

fn extrapolate_previous_values(history: &Vec<i64>) -> i64 {
    let mut differences = calculate_differences(history);
    let mut first_elements: Vec<i64> = vec![*differences.first().unwrap()];

    while !differences.iter().all(|&x| x == 0) {
        differences = calculate_differences(&differences);
        first_elements.push(*differences.first().unwrap());
    }

    let result = first_elements.iter().rev().fold(0, |acc, num| num - acc);

    history.first().unwrap() - result
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
    fn d09p2_partial_input1() {
        let input = "0 3 6 9 12 15";

        assert_eq!(part2(input), -3);
    }

    #[test]
    fn d09p2_partial_input2() {
        let input = "1 3 6 10 15 21";

        assert_eq!(part2(input), 0);
    }

    #[test]
    fn d09p2_partial_input3() {
        let input = "10 13 16 21 30 45";

        assert_eq!(part2(input), 5);
    }

    #[test]
    fn d09p2_full_input() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(part2(input), 2);
    }
}
