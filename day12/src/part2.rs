use std::time::Instant;

mod parser;

fn main() {
    let start_time = Instant::now();
    let input_file = include_str!("../input_p1.txt");
    let solution = part1(input_file);
    let execution_time = start_time.elapsed().as_secs_f64();
    println!("[{execution_time:?} seconds] {solution}");
}

fn part1(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d12p2_full_input() {
        let input = "";

        assert_eq!(part1(input), 0);
    }
}
