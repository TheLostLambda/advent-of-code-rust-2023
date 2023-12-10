use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;
advent_of_code::solution!(5);

fn build_map<'a>(lines: &[&'a str]) -> (&'a str, (&'a str, HashMap<u32, u32>)) {
    let (_, [from, to]) = Regex::new(r"(\w+)-to-(\w+) map:")
        .unwrap()
        .captures(lines[0])
        .unwrap()
        .extract();
    let mapping = lines[1..]
        .iter()
        .flat_map(|l| build_range_mapping(l))
        .collect();
    (from, (to, mapping))
}

fn build_range_mapping(line: &str) -> Vec<(u32, u32)> {
    let [dest, src, len] = Regex::new(r"(\d+) (\d+) (\d+)")
        .unwrap()
        .captures(line)
        .unwrap()
        .extract()
        .1
        .map(|c| c.parse().unwrap());
    (src..src + len).zip(dest..dest + len).collect()
}

fn seed_to_endpoint(mappings: &HashMap<&str, (&str, HashMap<u32, u32>)>, mut seed: u32) -> u32 {
    let mut current_type = "seed";
    while let Some((new_type, mapping)) = mappings.get(current_type) {
        current_type = new_type;
        seed = *mapping.get(&seed).unwrap_or(&seed);
    }
    seed
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let seeds: Vec<u32> = Regex::new(r"\d+")
        .unwrap()
        .find_iter(lines.next().unwrap())
        .map(|m| m.as_str().parse().unwrap())
        .collect();
    let mappings: HashMap<_, _> = lines
        .collect_vec()
        .split(|l| l.is_empty())
        .filter(|m| !m.is_empty())
        .map(|t| build_map(t))
        .collect();
    seeds.iter().map(|s| seed_to_endpoint(&mappings, *s)).min()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
