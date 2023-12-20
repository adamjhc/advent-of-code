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
                if (0..row).rev().zip(row..pattern.len()).fold(
                    0,
                    |rows_off_by_one, (top, bottom)| {
                        rows_off_by_one
                            + pattern[top]
                                .iter()
                                .zip(pattern[bottom].iter())
                                .filter(|(a, b)| a != b)
                                .count()
                    },
                ) == 1
                {
                    return 100 * row;
                }
            }

            for col in 1..pattern[0].len() {
                if (0..col).rev().zip(col..pattern[0].len()).fold(
                    0,
                    |cols_off_by_one, (left, right)| {
                        cols_off_by_one
                            + pattern.iter().filter(|row| row[left] != row[right]).count()
                    },
                ) == 1
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
        assert_eq!(result, 400)
    }

    #[test]
    fn solves_example_reddit() {
        // input taken from https://www.reddit.com/r/adventofcode/comments/18hitog/2023_day_13_easy_additional_examples/
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
#....#..#

.#.##.#.#
.##..##..
.#.##.#..
#......##
#......##
.#.##.#..
.##..##.#

#..#....#
###..##..
.##.#####
.##.#####
###..##..
#..#....#
#..##...#

#.##..##.
..#.##.#.
##..#...#
##...#..#
..#.##.#.
..##..##.
#.#.##.#."
            .to_string();
        let result = solve(&input);
        assert_eq!(result, 1400)
    }
}
