fn main() {
    let input = include_str!("input.txt");
    dbg!(solve(input));
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let game = line.split_once(':').unwrap().1;
            let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);
            game.split(&[';', ','][..]).for_each(|colour| {
                let (amount, colour) = colour.trim().split_once(' ').unwrap();
                let amount = amount.parse::<usize>().unwrap();
                match colour {
                    "red" if amount > min_red => min_red = amount,
                    "green" if amount > min_green => min_green = amount,
                    "blue" if amount > min_blue => min_blue = amount,
                    _ => (),
                }
            });

            min_red * min_green * min_blue
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
        assert_eq!(result, 2286)
    }
}
