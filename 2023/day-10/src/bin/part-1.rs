fn main() {
    let input = include_str!("input.txt");
    dbg!(solve(input));
}

#[derive(PartialEq)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NEBend,
    NWBend,
    SWBend,
    SEBend,
    Ground,
    Start,
}

impl Tile {
    fn parse(input: char) -> Self {
        match input {
            '|' => Self::VerticalPipe,
            '-' => Self::HorizontalPipe,
            'L' => Self::NEBend,
            'J' => Self::NWBend,
            '7' => Self::SWBend,
            'F' => Self::SEBend,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn go_from(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
        }
    }
}

fn solve(input: &str) -> usize {
    let tilemap: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| line.chars().map(Tile::parse).collect())
        .collect();

    let mut current_y = tilemap
        .iter()
        .position(|row| row.iter().any(|tile| tile == &Tile::Start))
        .unwrap();
    let mut current_x = tilemap[current_y]
        .iter()
        .position(|tile| tile == &Tile::Start)
        .unwrap();

    let mut direction = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .into_iter()
    .find(|direction| {
        let (x, y) = direction.go_from(current_x, current_y);
        matches!(
            (&tilemap[y][x], &direction),
            (Tile::VerticalPipe, Direction::South | Direction::North)
                | (Tile::HorizontalPipe, Direction::East | Direction::West)
                | (Tile::NEBend, Direction::South | Direction::West)
                | (Tile::NWBend, Direction::South | Direction::East)
                | (Tile::SWBend, Direction::North | Direction::East)
                | (Tile::SEBend, Direction::North | Direction::West)
        )
    })
    .expect("No direction from start usable");
    (current_x, current_y) = direction.go_from(current_x, current_y);
    let mut steps = 1;

    while tilemap[current_y][current_x] != Tile::Start {
        match (&tilemap[current_y][current_x], &direction) {
            (Tile::VerticalPipe, Direction::South) => (),
            (Tile::VerticalPipe, Direction::North) => (),
            (Tile::HorizontalPipe, Direction::East) => (),
            (Tile::HorizontalPipe, Direction::West) => (),
            (Tile::NEBend, Direction::South) => direction = Direction::East,
            (Tile::NEBend, Direction::West) => direction = Direction::North,
            (Tile::NWBend, Direction::South) => direction = Direction::West,
            (Tile::NWBend, Direction::East) => direction = Direction::North,
            (Tile::SWBend, Direction::North) => direction = Direction::West,
            (Tile::SWBend, Direction::East) => direction = Direction::South,
            (Tile::SEBend, Direction::North) => direction = Direction::East,
            (Tile::SEBend, Direction::West) => direction = Direction::South,
            _ => unreachable!("Something wrong in the direction instructions"),
        }

        (current_x, current_y) = direction.go_from(current_x, current_y);
        steps += 1;
    }

    steps / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_example() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
            .to_string();
        let result = solve(&input);
        assert_eq!(result, 4)
    }

    #[test]
    fn solves_example_2() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"
            .to_string();
        let result = solve(&input);
        assert_eq!(result, 8)
    }
}
