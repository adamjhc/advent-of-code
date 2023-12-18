use std::collections::HashSet;

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
    let mut loop_path = vec![(current_x, current_y)];

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
        loop_path.push((current_x, current_y));
    }

    // let mut stack = HashSet::new();
    // let mut visited = HashSet::new();
    // let mut inside = HashSet::new();
    // for y in 0..tilemap.len() {
    //     for x in 0..tilemap[0].len() {
    //         if visited.contains(&(x, y)) {
    //             continue;
    //         }

    //         visited.insert((x, y));
    //     }
    // }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_example() {
        let input = "...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ..........."
            .to_string();
        let result = solve(&input);
        assert_eq!(result, 4)
    }

    #[test]
    fn solves_example_2() {
        let input = ".F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ..."
            .to_string();
        let result = solve(&input);
        assert_eq!(result, 8)
    }

    #[test]
    fn solves_example_3() {
        let input = "FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L"
            .to_string();
        let result = solve(&input);
        assert_eq!(result, 10)
    }
}
