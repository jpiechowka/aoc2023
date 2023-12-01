fn main() {
    let input_file = include_str!("../input_p1.txt");
    let solution = part1(input_file);
    println!("{solution}");
}

fn part1(input: &str) -> u32 {
    let mut sum: u32 = 0;

    for line in input.lines() {
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
    use super::*;

    #[test]
    fn test() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(part1(input), 142);
    }
}
