fn main() {
    let input = include_str!("input.txt");
    dbg!(solve(input));
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (numbers_winning, numbers_ours) =
                line.split_once(':').unwrap().1.split_once('|').unwrap();
            let numbers_winning: Vec<&str> = numbers_winning.split_ascii_whitespace().collect();
            let numbers_ours: Vec<&str> = numbers_ours.split_ascii_whitespace().collect();

            let winning_count = numbers_winning
                .iter()
                .filter(|winning_number| numbers_ours.contains(winning_number))
                .count();

            if winning_count > 0 {
                2usize.pow(winning_count as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .to_string();
        let result = solve(&input);
        assert_eq!(result, 13)
    }
}
