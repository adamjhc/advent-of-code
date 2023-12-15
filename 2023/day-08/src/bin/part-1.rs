use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    dbg!(solve(input));
}

fn solve(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    let map: HashMap<&str, (&str, &str)> = lines
        .skip(1)
        .map(|line| {
            let (source, desintations) = line.split_once('=').unwrap();
            (source.trim(), (&desintations[2..5], &desintations[7..10]))
        })
        .collect();

    let mut current = "AAA";
    let mut steps = 0;
    let mut instructions = instructions.iter().cycle();
    while current != "ZZZ" {
        let next_steps = map.get(current).unwrap();
        current = match instructions.next().unwrap() {
            'L' => next_steps.0,
            'R' => next_steps.1,
            _ => panic!(),
        };
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_example() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            .to_string();
        let result = solve(&input);
        assert_eq!(result, 2)
    }
}
