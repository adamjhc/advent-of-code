fn main() {
    let input = include_str!("input.txt");
    dbg!(solve(input));
}

fn solve(input: &str) -> usize {
    let time_and_distance: Vec<usize> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .collect::<Vec<&str>>()
                .join("")
                .parse()
                .unwrap()
        })
        .collect();

    let time = time_and_distance[0];
    let distance = time_and_distance[1];
    (0..=time)
        .filter(|time_held| (time - time_held) * time_held > distance)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_example() {
        let input = "Time:      7  15   30
Distance:  9  40  200"
            .to_string();
        let result = solve(&input);
        assert_eq!(result, 71503)
    }
}
