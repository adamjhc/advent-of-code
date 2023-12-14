fn main() {
    let input = include_str!("input.txt");
    println!("{}", solve(input));
}

#[derive(Clone)]
struct Mapping {
    source_min: usize,
    source_max: usize,
    destination_min: usize,
}

struct Seed {
    start: usize,
    length: usize,
}

fn solve(input: &str) -> usize {
    let mut lines = input.lines();
    let seed_ranges: Vec<usize> = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect();
    let seeds: Vec<Seed> = seed_ranges
        .chunks(2)
        .map(|seed| Seed {
            start: seed[0],
            length: seed[1],
        })
        .collect();

    lines.next();
    let mut map_no = 0;
    let mut maps: Vec<Vec<Mapping>> = vec![vec![]; 7];
    for line in lines {
        if line.starts_with(|char: char| char.is_alphabetic()) {
            continue;
        }

        if line.is_empty() {
            map_no += 1;
            continue;
        }

        let nums: Vec<usize> = line
            .split_ascii_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let map = maps.get_mut(map_no).unwrap();
        map.push(Mapping {
            source_min: nums[1],
            source_max: nums[1] + nums[2],
            destination_min: nums[0],
        });
    }

    seeds
        .iter()
        .map(|seed| {
            (seed.start..seed.start + seed.length)
                .map(|seed| {
                    let mut num = seed;
                    for map in &maps {
                        if let Some(mapping) = map
                            .iter()
                            .find(|mapping| mapping.source_min <= num && num < mapping.source_max)
                        {
                            num = mapping.destination_min + (num - mapping.source_min);
                        }
                    }
                    num
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_example() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            .to_string();
        let result = solve(&input);
        assert_eq!(result, 46)
    }
}
