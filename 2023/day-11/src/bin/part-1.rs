fn main() {
    let input = include_str!("input.txt");
    dbg!(solve(input));
}

fn _print_universe(universe: &[Vec<char>]) {
    universe
        .iter()
        .for_each(|row| println!("{}", row.iter().collect::<String>()));
    println!("-------------")
}

fn solve(input: &str) -> usize {
    let mut universe = input.lines().fold(Vec::new(), |mut rows, line| {
        let chars: Vec<char> = line.chars().collect();
        if chars.iter().all(|char| char == &'.') {
            rows.push(chars.clone());
        }
        rows.push(chars);
        rows
    });

    let mut cols_added = 0;
    let num_rows = universe.len();
    for col in 0..universe[0].len() {
        if universe[0..num_rows]
            .iter()
            .all(|row| row[col + cols_added] == '.')
        {
            universe[0..num_rows].iter_mut().for_each(|row| {
                row.insert(col + cols_added, '.');
            });
            cols_added += 1;
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
            sum_lengths +=
                galaxies[i].0.abs_diff(galaxies[j].0) + galaxies[i].1.abs_diff(galaxies[j].1);
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
        let result = solve(&input);
        assert_eq!(result, 374)
    }
}
