fn main() {
    let input = include_str!("input.txt");
    dbg!(solve(input));
}

fn solve(input: &str) -> usize {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    input
        .lines()
        .map(|line| {
            let (number, game) = line.split_once(':').unwrap();
            let number = number.split_once(' ').unwrap().1.parse::<usize>().unwrap();
            if game.split(&[';', ','][..]).any(|colour| {
                let (amount, colour) = colour.trim().split_once(' ').unwrap();
                let amount = amount.parse::<usize>().unwrap();
                match colour {
                    "red" if amount > max_red => true,
                    "green" if amount > max_green => true,
                    "blue" if amount > max_blue => true,
                    _ => false,
                }
            }) {
                0
            } else {
                number
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string();
        let result = solve(&input);
        assert_eq!(result, 8)
    }
}
