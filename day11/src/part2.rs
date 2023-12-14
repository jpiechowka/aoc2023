use std::time::Instant;

mod parser;

fn main() {
    let start_time = Instant::now();
    let input_file = include_str!("../input_p1.txt");
    let solution = part2(input_file);
    let execution_time = start_time.elapsed().as_secs_f64();
    println!("[{execution_time:?} seconds] {solution}");
}

fn part2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d11p2_input1() {
        let input = "";

        assert_eq!(part2(input), 0);
    }

    #[test]
    fn d11p2_input2() {
        let input = "";

        assert_eq!(part2(input), 0);
    }
}
