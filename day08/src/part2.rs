use std::time::Instant;

use crate::parser::{parse_input, Instruction};

mod parser;

fn main() {
    let start_time = Instant::now();
    let input_file = include_str!("../input_p2.txt");
    let solution = part2(input_file);
    let execution_time = start_time.elapsed().as_secs_f64();
    println!("[{execution_time:?} seconds] {solution}");
}

fn part2(input: &str) -> u64 {
    let (_, (instructions, map)) = parse_input(input).expect("should parse input");

    let starting_nodes: Vec<&str> = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .collect();

    let results = starting_nodes
        .iter()
        .map(|node| {
            let mut visited_nodes = vec![*node];
            let mut current_node = *node;

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
                    if next_node.ends_with('Z') {
                        Some(idx + 1)
                    } else {
                        visited_nodes.push(next_node);
                        current_node = next_node;
                        None
                    }
                })
                .expect("should find a cycle")
        })
        .collect::<Vec<usize>>();

    lcm(&results) as u64
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }

    let a = nums[0];
    let b = lcm(&nums[1..]);

    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d08p2_full_input() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(part2(input), 6);
    }
}
