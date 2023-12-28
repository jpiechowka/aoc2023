use std::collections::HashMap;
use std::time::Instant;

use rayon::prelude::*;

use crate::parser::{arrangements_count_with_cache, parse_input, PuzzleLine};

mod parser;

fn main() {
    let start_time = Instant::now();
    let input_file = include_str!("../input_p1.txt");
    let solution = part2(input_file);
    let execution_time = start_time.elapsed().as_secs_f64();
    println!("[{execution_time:?} seconds] {solution}");
}

fn part2(input: &str) -> usize {
    let (_, parsed_input) = parse_input(input).expect("should parse input");

    let unfolded_tiles: Vec<PuzzleLine> = parsed_input
        .iter()
        .map(|puzzle_line| puzzle_line.unfold_records())
        .collect();

    unfolded_tiles
        .par_iter()
        .map(|puzzle_line| {
            arrangements_count_with_cache(
                &puzzle_line.tiles,
                &puzzle_line.arrangements,
                &mut HashMap::new(),
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d12p1_full_input() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(part2(input), 525152);
    }
}
