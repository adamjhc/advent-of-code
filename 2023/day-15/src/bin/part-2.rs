use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    dbg!(solve(input));
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |value, char| ((value + char as usize) * 17) % 256)
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .fold(
            HashMap::new(),
            |mut boxes: HashMap<usize, Vec<(&str, usize)>>, step| {
                if let Some((label, _)) = step.split_once('-') {
                    if let Some(boxa) = boxes.get_mut(&hash(label)) {
                        if let Some(index) = boxa.iter().position(|lens| lens.0 == label) {
                            boxa.remove(index);
                        }
                    }
                } else if let Some((label, focal_length)) = step.split_once('=') {
                    let lenses = boxes.entry(hash(label)).or_default();
                    let focal_length = focal_length.parse().unwrap();
                    if let Some(index) = lenses.iter().position(|lens| lens.0 == label) {
                        lenses[index].1 = focal_length;
                    } else {
                        lenses.push((label, focal_length));
                    }
                }

                boxes
            },
        )
        .iter()
        .map(|(box_no, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(i, lens)| (box_no + 1) * (i + 1) * lens.1)
                .sum::<usize>()
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
        assert_eq!(result, 145)
    }
}
