use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    dbg!(solve(input));
}

fn solve(input: &str) -> usize {
    let mut cards_count = 0;
    let mut to_process = vec![];
    let mut card_outputs: HashMap<usize, Vec<usize>> = HashMap::new();

    input.lines().enumerate().for_each(|(i, line)| {
        let (numbers_winning, numbers_ours) =
            line.split_once(':').unwrap().1.split_once('|').unwrap();
        let numbers_winning: Vec<&str> = numbers_winning.split_ascii_whitespace().collect();
        let numbers_ours: Vec<&str> = numbers_ours.split_ascii_whitespace().collect();

        let winning_count = numbers_winning
            .iter()
            .filter(|winning_number| numbers_ours.contains(winning_number))
            .count();

        cards_count += 1;
        let card_number = i + 1;
        let new_cards: Vec<usize> = (1..=winning_count).map(|j| card_number + j).collect();
        card_outputs.insert(card_number, new_cards.clone());
        to_process.extend(new_cards);
    });

    while !to_process.is_empty() {
        cards_count += 1;
        let card = to_process.pop().unwrap();
        to_process.extend(card_outputs.get(&card).unwrap())
    }

    cards_count
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
        assert_eq!(result, 30)
    }
}
