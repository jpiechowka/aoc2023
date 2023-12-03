fn main() {
    let input_file = include_str!("../input_p2.txt");
    let solution = part2(input_file);
    println!("{solution}");
}

fn part2(input: &str) -> u32 {
    let parsed_input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut sum: u32 = 0;

    for (line_idx, line) in parsed_input.iter().enumerate() {
        for (char_idx, char) in line.iter().enumerate() {
            if *char != '*' {
                continue;
            }
            sum += gear_ratios(line_idx, char_idx, &parsed_input);
        }
    }

    sum
}

fn gear_ratios(line_idx: usize, char_idx: usize, input: &Vec<Vec<char>>) -> u32 {
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
    let mut adjacent_numbers: Vec<u32> = Vec::new();

    for (ai, aj) in adjacent_idxs.iter() {
        let adjacent_i = line_idx as i32 + ai;
        let adjacent_j = char_idx as i32 + aj;

        if adjacent_i >= 0
            && adjacent_i < input.len() as i32
            && adjacent_j >= 0
            && adjacent_j < input[0].len() as i32
        {
            let number = find_number_in_line(adjacent_j as usize, &input[adjacent_i as usize]);
            adjacent_numbers.push(number);
        }
    }

    adjacent_numbers.retain(|&x| x != 0);
    adjacent_numbers.dedup();
    if adjacent_numbers.len() < 2 {
        return 0;
    }

    adjacent_numbers.iter().product()
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

    output.parse::<u32>().unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d03p2_small_input() {
        let input = "467..114..
...*......
..35..633.";

        assert_eq!(part2(input), 16345);
    }

    #[test]
    fn d03p2_full_input() {
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

        assert_eq!(part2(input), 467835);
    }
}
