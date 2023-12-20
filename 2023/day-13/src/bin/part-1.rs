fn main() {
    let input = include_str!("input.txt");
    dbg!(solve(input));
}

fn _print_pattern(universe: &[Vec<char>]) {
    universe
        .iter()
        .for_each(|row| println!("{}", row.iter().collect::<String>()));
    println!("-------------")
}

fn solve(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern_raw| {
            let pattern: Vec<Vec<char>> = pattern_raw
                .lines()
                .map(|line| line.chars().collect())
                .collect();

            for row in 1..pattern.len() {
                if (0..row)
                    .rev()
                    .zip(row..pattern.len())
                    .all(|(top, bottom)| pattern[top] == pattern[bottom])
                {
                    return 100 * row;
                }
            }

            for col in 1..pattern[0].len() {
                if (0..col)
                    .rev()
                    .zip(col..pattern[0].len())
                    .all(|(left, right)| pattern.iter().all(|row| row[left] == row[right]))
                {
                    return col;
                }
            }

            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_example() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            .to_string();
        let result = solve(&input);
        assert_eq!(result, 405)
    }
}
