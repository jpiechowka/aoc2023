fn main() {
    let input_file = include_str!("../input_p1.txt");
    let solution = part1(input_file);
    println!("{solution}");
}

fn part1(input: &str) -> u32 {
    let parsed_input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut sum: u32 = 0;

    for (line_idx, line) in parsed_input.iter().enumerate() {
        // Apparently the same number can exist multiple times in one line.
        // HashSet could be used otherwise. In this case .dedup() should be sufficient.
        let mut numbers: Vec<u32> = Vec::new();

        for (char_idx, _char) in line.iter().enumerate() {
            if is_near_symbol(line_idx, char_idx, &parsed_input) {
                numbers.push(find_number_in_line(char_idx, line));
            }
        }

        numbers.dedup();
        sum += numbers.iter().sum::<u32>();
    }

    sum
}

fn is_near_symbol(line_idx: usize, char_idx: usize, input: &Vec<Vec<char>>) -> bool {
    let adjacent_idxs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (ai, aj) in adjacent_idxs.iter() {
        let adjacent_i = line_idx as i32 + ai;
        let adjacent_j = char_idx as i32 + aj;

        if adjacent_i >= 0
            && adjacent_i < input.len() as i32
            && adjacent_j >= 0
            && adjacent_j < input[0].len() as i32
            && is_symbol_except_period(input[adjacent_i as usize][adjacent_j as usize])
        {
            return true;
        }
    }

    false
}

fn find_number_in_line(char_idx: usize, input: &Vec<char>) -> u32 {
    let current_char = input[char_idx];
    if !current_char.is_ascii_digit() {
        return 0;
    }

    let mut output = String::new();

    // Iterate left to right in line
    for ai in -2..=2 {
        // Seems like 3 digit numbers are max
        let adjacent_i = char_idx as i32 + ai;

        if adjacent_i >= 0 && adjacent_i < input.len() as i32 {
            let char = input[adjacent_i as usize];
            if char.is_ascii_digit() {
                output.push(char);
            }
            if !char.is_ascii_digit() && ai < 0 {
                output.clear();
                continue;
            }
            if !char.is_ascii_digit() && ai > 0 {
                break;
            }
        }
    }

    output.parse::<u32>().unwrap_or(0)
}

fn is_symbol_except_period(c: char) -> bool {
    c.is_ascii_punctuation() && c != '.'
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0, 0)]
    #[case(1, 0)]
    #[case(2, 123)]
    #[case(3, 123)]
    #[case(4, 123)]
    #[case(5, 0)]
    #[case(6, 0)]
    fn d03p1_lines_middle(#[case] index: usize, #[case] expected: u32) {
        let test_line: Vec<char> = vec!['.', '.', '1', '2', '3', '.', '.'];
        assert_eq!(expected, find_number_in_line(index, &test_line));
    }

    #[rstest]
    #[case(0, 123)]
    #[case(1, 123)]
    #[case(2, 123)]
    #[case(3, 0)]
    #[case(4, 0)]
    #[case(5, 0)]
    #[case(6, 0)]
    fn d03p1_lines_front(#[case] index: usize, #[case] expected: u32) {
        let test_line: Vec<char> = vec!['1', '2', '3', '.', '.', '.', '.'];
        assert_eq!(expected, find_number_in_line(index, &test_line));
    }

    #[rstest]
    #[case(0, 0)]
    #[case(1, 0)]
    #[case(2, 0)]
    #[case(3, 0)]
    #[case(4, 123)]
    #[case(5, 123)]
    #[case(6, 123)]
    fn d03p1_lines_end(#[case] index: usize, #[case] expected: u32) {
        let test_line: Vec<char> = vec!['.', '.', '.', '.', '1', '2', '3'];
        assert_eq!(expected, find_number_in_line(index, &test_line));
    }

    #[rstest]
    #[case(0, 12)]
    #[case(1, 12)]
    #[case(2, 0)]
    #[case(3, 0)]
    #[case(4, 123)]
    #[case(5, 123)]
    #[case(6, 123)]
    fn d03p1_lines_different_numbers(#[case] index: usize, #[case] expected: u32) {
        let test_line: Vec<char> = vec!['1', '2', '.', '.', '1', '2', '3'];
        assert_eq!(expected, find_number_in_line(index, &test_line));
    }

    #[test]
    fn d03p1_full_input() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part1(input), 4361);
    }
}
