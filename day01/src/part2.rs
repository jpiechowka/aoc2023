fn main() {
    let input_file = include_str!("../input_p2.txt");
    let solution = part2(input_file);
    println!("{solution}");
}

fn part2(input: &str) -> u32 {
    let mut sum: u32 = 0;

    for line in input.lines() {
        // Digits can overlap. Account for that by using this hacky way
        let line = line.replace("one", "o1e");
        let line = line.replace("two", "t2o");
        let line = line.replace("three", "th3ee");
        let line = line.replace("four", "f4ur");
        let line = line.replace("five", "f5ve");
        let line = line.replace("six", "s6x");
        let line = line.replace("seven", "se7en");
        let line = line.replace("eight", "ei8ht");
        let line = line.replace("nine", "n9ne");

        let first_digit = line.chars().filter_map(|c| c.to_digit(10)).next();

        let last_digit = line.chars().filter_map(|c| c.to_digit(10)).last();

        let num: String = format!("{}{}", first_digit.unwrap_or(0), last_digit.unwrap_or(0));
        let num_parsed: u32 = num.parse().unwrap_or(0);
        sum += num_parsed;
    }

    sum
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn line_test(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(expected, part2(line))
    }

    #[test]
    fn test() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(part2(input), 281);
    }
}
