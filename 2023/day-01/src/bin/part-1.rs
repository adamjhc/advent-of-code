fn main() {
    let input = include_str!("input.txt");
    dbg!(solve(input));
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut first_digit = None;
            let mut second_digit = None;
            line.chars().for_each(|char| {
                if char.is_numeric() {
                    if first_digit.is_none() {
                        first_digit = Some(char);
                    }
                    second_digit = Some(char);
                }
            });

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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            .to_string();
        let result = solve(&input);
        assert_eq!(result, 142)
    }
}
