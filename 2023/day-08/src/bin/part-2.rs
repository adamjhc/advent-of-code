use std::collections::HashMap;

use num::Integer;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", solve(input));
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

    let mut currents: Vec<&str> = map
        .clone()
        .into_keys()
        .filter(|name| name.ends_with('A'))
        .collect();
    let steps: Vec<usize> = currents
        .iter_mut()
        .map(|current| {
            let mut steps = 0;
            let mut instructions = instructions.iter().cycle();
            while !current.ends_with('Z') {
                let next_steps = map.get(current).unwrap();
                *current = match instructions.next().unwrap() {
                    'L' => next_steps.0,
                    'R' => next_steps.1,
                    _ => panic!(),
                };
                steps += 1;
            }

            steps
        })
        .collect();

    steps.iter().fold(1, |lcm, step| step.lcm(&lcm))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_example() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            .to_string();
        let result = solve(&input);
        assert_eq!(result, 6)
    }
}
