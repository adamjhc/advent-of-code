fn main() {
    let input = include_str!("input.txt");
    dbg!(solve(input, 1_000_000));
}

fn _print_universe(universe: &[Vec<char>]) {
    universe
        .iter()
        .for_each(|row| println!("{}", row.iter().collect::<String>()));
    println!("-------------")
}

fn solve(input: &str, expansion_amount: usize) -> usize {
    let mut universe = input.lines().fold(Vec::new(), |mut rows, line| {
        let chars: Vec<char> = line.chars().collect();
        if chars.iter().all(|char| char == &'.') {
            rows.push(vec!['@'; chars.len()]);
        } else {
            rows.push(chars);
        }
        rows
    });

    let num_rows = universe.len();
    for col in 0..universe[0].len() {
        if universe[0..num_rows]
            .iter()
            .all(|row| row[col] == '.' || row[col] == '@')
        {
            universe[0..num_rows].iter_mut().for_each(|row| {
                row[col] = '@';
            });
        }
    }

    let mut galaxies = Vec::new();
    for row in 0..num_rows {
        for col in 0..universe[0].len() {
            if universe[row][col] == '#' {
                galaxies.push((col, row));
            }
        }
    }

    let mut sum_lengths = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (galaxy_a_col, galaxy_a_row) = galaxies[i];
            let (galaxy_b_col, galaxy_b_row) = galaxies[j];
            let expansion_row_count = universe
                [galaxy_a_row.min(galaxy_b_row)..galaxy_a_row.max(galaxy_b_row)]
                .iter()
                .filter(|row| row[0] == '@')
                .count();
            let expansion_col_count = universe[0]
                [galaxy_a_col.min(galaxy_b_col)..galaxy_a_col.max(galaxy_b_col)]
                .iter()
                .filter(|char| char == &&'@')
                .count();

            sum_lengths += (galaxy_a_col.abs_diff(galaxy_b_col) - expansion_col_count)
                + (expansion_col_count * expansion_amount)
                + (galaxy_a_row.abs_diff(galaxy_b_row) - expansion_row_count)
                + (expansion_row_count * expansion_amount);
        }
    }

    sum_lengths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_example() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            .to_string();
        let result = solve(&input, 10);
        assert_eq!(result, 1030)
    }

    #[test]
    fn solves_example_2() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            .to_string();
        let result = solve(&input, 100);
        assert_eq!(result, 8410)
    }
}
