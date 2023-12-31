use std::time::Instant;

use rayon::prelude::*;

use crate::parser::parse_almanac_part2;

mod parser;

fn main() {
    let start_time = Instant::now();
    let input_file = include_str!("../input_p2.txt");
    let solution = part2(input_file);
    let execution_time = start_time.elapsed().as_secs_f64();
    println!("[{execution_time:?} seconds] {solution}");
}

fn part2(input: &str) -> u64 {
    let (_input, (seeds, mappings)) = parse_almanac_part2(input).expect("should parse input");
    let minimum_location = seeds
        .into_par_iter()
        .flat_map(|range| range.clone())
        .map(|seed| mappings.iter().fold(seed, |seed, map| map.translate(seed)))
        .min();

    minimum_location.expect("should have minimum value")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d05p2_full_input() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(part2(input), 46);
    }
}
