fn main() {
    let input_file = include_str!("../input_p1.txt");
    let solution = part1(input_file);
    println!("{solution}");
}

fn part1(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d04p1_full_input() {
        let input = "";

        assert_eq!(part1(input), 0);
    }
}
