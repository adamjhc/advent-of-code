fn main() {
    let input = include_str!("input.txt");
    dbg!(solve(input));
}

fn solve(input: &str) -> usize {
    let numbers = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    input
        .lines()
        .map(|line| {
            let mut first_digit = None;
            let mut second_digit = None;
            for i in 0..line.len() {
                let char = line.chars().nth(i).unwrap();
                if char.is_numeric() {
                    if first_digit.is_none() {
                        first_digit = Some(char);
                    }
                    second_digit = Some(char);
                } else {
                    let mut number_text = None;
                    for j in 0..numbers.len() {
                        let number = numbers[j];
                        if (line.len() as isize - i as isize - number.0.len() as isize) < 0 {
                            continue;
                        }

                        if number.0.eq(&line[i..i + number.0.len()]) {
                            number_text = Some(number.1);
                            break;
                        }
                    }

                    if let Some(number_text) = number_text {
                        if first_digit.is_none() {
                            first_digit = Some(number_text);
                        }
                        second_digit = Some(number_text);
                    }
                }
            }

            format!("{}{}", first_digit.unwrap(), second_digit.unwrap())
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_example() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .to_string();
        let result = solve(&input);
        assert_eq!(result, 281)
    }
}
