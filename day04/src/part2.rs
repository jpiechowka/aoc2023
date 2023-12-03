fn main() {
    let input_file = include_str!("../input_p2.txt");
    let solution = part2(input_file);
    println!("{solution}");
}

fn part2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d04p2_full_input() {
        let input = "";

        assert_eq!(part2(input), 0);
    }
}
