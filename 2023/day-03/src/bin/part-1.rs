fn main() {
    let input = include_str!("input.txt");
    dbg!(solve(input));
}

fn solve(input: &str) -> usize {
    // loop through every character
    // if a number
    //      find end of number
    //      check edges around number for a symbol
    //      if symbol then add to sum
    //  sum

    let mut sum_part_numbers = 0;
    let mut already_parsed = false;
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let line_length = lines[0].len();
    let lines_num = lines.len();
    for y in 0..lines_num {
        for x in 0..line_length {
            if !already_parsed && lines[y][x].is_ascii_digit() {
                let mut end = x as isize;
                while end < line_length as isize - 1 && lines[y][end as usize + 1].is_ascii_digit()
                {
                    end += 1;
                }

                let part_number = lines[y][x..=end as usize]
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();

                let min_x = (x as isize - 1).max(0) as usize;
                let max_x = (end as usize + 1).min(line_length - 1);
                let is_symbol = |&x: &char| x.is_ascii_punctuation() && x != '.';
                if (y != 0 && lines[y - 1][min_x..=max_x].iter().any(is_symbol))
                    || (is_symbol(&lines[y][min_x]) || is_symbol(&lines[y][max_x]))
                    || (y != lines_num - 1 && lines[y + 1][min_x..=max_x].iter().any(is_symbol))
                {
                    sum_part_numbers += part_number;
                    already_parsed = true;
                    if part_number == 682 {
                        dbg!(&part_number);
                    }
                    continue;
                }
            }

            if lines[y][x].is_ascii_punctuation() {
                already_parsed = false;
            }
        }

        already_parsed = false;
    }

    sum_part_numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .to_string();
        let result = solve(&input);
        assert_eq!(result, 4361)
    }
}
