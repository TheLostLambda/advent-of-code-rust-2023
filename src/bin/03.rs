use std::ops::{Range, RangeInclusive};

use regex::Regex;

advent_of_code::solution!(3);

// FIXME: This solution makes two passes over the input... I'm certain we can
// do better eventually!

fn extract_symbols(lines: &[(usize, &str)]) -> Vec<(usize, usize)> {
    let symbol = Regex::new(r"[^\d\.]").unwrap();
    lines
        .iter()
        .flat_map(|(r, s)| symbol.find_iter(s).map(|m| (*r, m.start())))
        .collect::<Vec<_>>()
}

// FIXME: Should probably be using structs here...
fn extract_numbers(
    lines: &[(usize, &str)],
) -> Vec<(u32, (RangeInclusive<usize>, RangeInclusive<usize>))> {
    let symbol = Regex::new(r"\d+").unwrap();
    lines
        .iter()
        .flat_map(|(r, s)| {
            symbol.find_iter(s).map(move |m| {
                (
                    m.as_str().parse::<u32>().unwrap(),
                    adjacent_ranges(*r, m.start(), m.end()),
                )
            })
        })
        .collect::<Vec<_>>()
}

fn adjacent_ranges(
    row: usize,
    start: usize,
    end: usize,
) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    (
        row.saturating_sub(1)..=row.saturating_add(1),
        start.saturating_sub(1)..=end,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.lines().enumerate().collect();
    let symbol_coordinates = extract_symbols(&lines);
    Some(
        extract_numbers(&lines)
            .iter()
            .filter_map(|(n, (rr, cr))| {
                symbol_coordinates
                    .iter()
                    .any(|(r, c)| rr.contains(r) && cr.contains(c))
                    .then_some(n)
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
