use std::time::Instant;

use crate::parser::{parse_input, Instruction};

mod parser;

fn main() {
    let start_time = Instant::now();
    let input_file = include_str!("../input_p1.txt");
    let solution = part1(input_file);
    let execution_time = start_time.elapsed().as_secs_f64();
    println!("[{execution_time:?} seconds] {solution}");
}

fn part1(input: &str) -> u64 {
    let (_, (instructions, map)) = parse_input(input).expect("should parse input");
    let mut current_node = "AAA";
    let Some(step_count) =
        instructions
            .iter()
            .cycle()
            .enumerate()
            .find_map(|(idx, instruction)| {
                let directions = map
                    .get(current_node)
                    .expect("should get current node from map");
                let next_node = match instruction {
                    Instruction::Right => directions.1,
                    Instruction::Left => directions.0,
                };
                if next_node == "ZZZ" {
                    Some(idx + 1)
                } else {
                    current_node = next_node;
                    None
                }
            })
    else {
        panic!()
    };

    step_count as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d08p1_first_input() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(input), 2);
    }

    #[test]
    fn d08p1_second_input() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(input), 6);
    }
}
