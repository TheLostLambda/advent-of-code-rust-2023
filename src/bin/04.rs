use std::collections::HashSet;

use regex::Regex;

advent_of_code::solution!(4);

fn score_scratchcard(winning_numbers: HashSet<u32>, our_numbers: HashSet<u32>) -> u32 {
    let wins = winning_numbers.intersection(&our_numbers).count() as u32;
    if wins == 0 {
        0
    } else {
        2_u32.pow(wins - 1)
    }
}

fn parse_numbers(input: &str) -> HashSet<u32> {
    Regex::new(r"\d+")
        .unwrap()
        .find_iter(input)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        Regex::new(r":([^\|]+)\|([^\n]+)")
            .unwrap()
            .captures_iter(input)
            .map(|c| {
                score_scratchcard(
                    parse_numbers(c.get(1).unwrap().as_str()),
                    parse_numbers(c.get(2).unwrap().as_str()),
                )
            })
            .sum(),
    )
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
