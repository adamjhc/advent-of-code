fn main() {
    let input = include_str!("input.txt");
    dbg!(solve(input));
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|step| {
            step.chars()
                .fold(0, |value, char| ((value + char as usize) * 17) % 256)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_example() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string();
        let result = solve(&input);
        assert_eq!(result, 1320)
    }
}
