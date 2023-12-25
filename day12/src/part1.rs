use std::time::Instant;

use crate::parser::{parse_input, PuzzleLine};

mod parser;

fn main() {
    let start_time = Instant::now();
    let input_file = include_str!("../input_p1.txt");
    let solution = part1(input_file);
    let execution_time = start_time.elapsed().as_secs_f64();
    println!("[{execution_time:?} seconds] {solution}");
}

fn part1(input: &str) -> u64 {
    let (_, parsed_input) = parse_input(input).expect("should parse input");
    parsed_input.iter().map(get_arrangements_count).sum()
}

fn get_arrangements_count(puzzle_line: &PuzzleLine) -> u64 {
    let permutations = puzzle_line.generate_available_permutations();
    permutations
        .iter()
        .filter(|permutation| permutation.is_tiles_arrangement_correct())
        .count() as u64
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::parser::parse_line;

    use super::*;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    fn d12p1_arrangements_test(#[case] input_line: &str, #[case] expected_arrangements_count: u64) {
        let (_, parsed_input) = parse_line(input_line).expect("should parse line");
        assert_eq!(
            get_arrangements_count(&parsed_input),
            expected_arrangements_count
        );
    }

    #[test]
    fn d12p1_full_input() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(part1(input), 21);
    }
}
