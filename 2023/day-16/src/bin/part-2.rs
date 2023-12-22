use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    dbg!(solve(input));
}

enum Tile {
    EmptySpace,
    SlashMirror,
    BackslashMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::EmptySpace,
            '/' => Self::SlashMirror,
            '\\' => Self::BackslashMirror,
            '|' => Self::VerticalSplitter,
            '-' => Self::HorizontalSplitter,
            _ => unreachable!("character not recognised"),
        }
    }
}

#[derive(PartialEq, Debug, Hash, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn solve(input: &str) -> usize {
    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(|char| char.into()).collect())
        .collect();

    let starting_positions = (0..grid.len())
        .flat_map(|row| {
            [
                ((row, 0), Direction::Right),
                ((row, grid[0].len() - 1), Direction::Left),
            ]
        })
        .chain((0..grid[0].len()).flat_map(|col| {
            [
                ((0, col), Direction::Down),
                ((grid.len() - 1, col), Direction::Up),
            ]
        }));

    starting_positions
        .map(|starting_position| {
            let y_max = grid.len() as isize;
            let x_max = grid[0].len() as isize;
            let mut beams = vec![starting_position];
            let mut energized_tiles = HashSet::new();
            let mut visited_tiles = HashSet::new();
            while let Some(((y, x), mut direction)) = beams.pop() {
                let (mut y, mut x) = (y as isize, x as isize);
                while (0 <= y && y < y_max) && (0 <= x && x < x_max) {
                    if visited_tiles.contains(&((y, x), direction)) {
                        break;
                    }
                    energized_tiles.insert((y, x));
                    visited_tiles.insert(((y, x), direction));

                    match (&grid[y as usize][x as usize], &direction) {
                        (Tile::SlashMirror, Direction::Right) => direction = Direction::Up,
                        (Tile::SlashMirror, Direction::Up) => direction = Direction::Right,
                        (Tile::SlashMirror, Direction::Down) => direction = Direction::Left,
                        (Tile::SlashMirror, Direction::Left) => direction = Direction::Down,
                        (Tile::BackslashMirror, Direction::Right) => direction = Direction::Down,
                        (Tile::BackslashMirror, Direction::Down) => direction = Direction::Right,
                        (Tile::BackslashMirror, Direction::Left) => direction = Direction::Up,
                        (Tile::BackslashMirror, Direction::Up) => direction = Direction::Left,
                        (Tile::VerticalSplitter, Direction::Right | Direction::Left) => {
                            beams.push(((y as usize + 1, x as usize), Direction::Down));
                            direction = Direction::Up;
                        }
                        (Tile::HorizontalSplitter, Direction::Up | Direction::Down) => {
                            beams.push(((y as usize, x as usize + 1), Direction::Right));
                            direction = Direction::Left;
                        }
                        _ => (),
                    }

                    (y, x) = match direction {
                        Direction::Up => (y - 1, x),
                        Direction::Down => (y + 1, x),
                        Direction::Left => (y, x - 1),
                        Direction::Right => (y, x + 1),
                    };
                }
            }

            energized_tiles.len()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_example() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
            .to_string();
        let result = solve(&input);
        assert_eq!(result, 51)
    }
}
