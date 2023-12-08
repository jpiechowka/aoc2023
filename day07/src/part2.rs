use std::time::Instant;

use crate::parser::parse_input_part2;

mod parser;

fn main() {
    let start_time = Instant::now();
    let input_file = include_str!("../input_p2.txt");
    let solution = part2(input_file);
    let execution_time = start_time.elapsed().as_secs_f64();
    println!("[{execution_time:?} seconds] {solution}");
}

fn part2(input: &str) -> u64 {
    let mut hands = parse_input_part2(input);

    hands.sort_by(|a, b| {
        b.hand_type
            .cmp(&a.hand_type)
            .then_with(|| b.cards.iter().cmp(a.cards.iter()))
    });

    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| {
            let rank = (hands.len() - idx) as u64;
            hand.bid * rank
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d07p2_full_input() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(part2(input), 5905);
    }
}
