fn main() {
    let input = include_str!("input.txt");
    dbg!(solve(input));
}

fn solve(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let readings: Vec<isize> = line
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let mut differences = vec![readings];
            let mut layer = 0;
            while differences[layer].iter().any(|&num| num != 0) {
                differences.push(
                    differences[layer]
                        .windows(2)
                        .map(|window| window[1] - window[0])
                        .collect(),
                );
                layer += 1;
            }

            while layer > 0 {
                let layer_first = *differences[layer].first().unwrap();
                let layer_first_above = *differences[layer - 1].first().unwrap();
                differences[layer - 1].insert(0, layer_first_above - layer_first);
                layer -= 1;
            }

            differences[0].first().copied().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_example() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            .to_string();
        let result = solve(&input);
        assert_eq!(result, 2)
    }
}
