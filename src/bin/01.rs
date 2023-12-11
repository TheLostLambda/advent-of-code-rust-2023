use once_cell::sync::Lazy;
use regex::Regex;

advent_of_code::solution!(1);

// OPTIMISATION: This is around 380x faster than building the regex fresh for every line!
static DIGIT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap());

fn process_line_one(line: &str) -> Option<u32> {
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    Some(10 * digits.first()? + digits.last()?)
}

fn convert_digit(digit_str: &str) -> Option<u32> {
    Some(match digit_str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        c => c.parse().ok()?,
    })
}

fn process_line_two(line: &str) -> Option<u32> {
    let first_digit = convert_digit(DIGIT_REGEX.find(line)?.as_str())?;
    let last_digit = convert_digit(
        (0..line.len())
            .rev()
            .find_map(|i| DIGIT_REGEX.find(&line[i..]))?
            .as_str(),
    )?;
    Some(10 * first_digit + last_digit)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().filter_map(process_line_one).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().filter_map(process_line_two).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
        assert_eq!(
            process_line_two("qccnqvfnkkkvsixktsixnine1twoneq"),
            Some(61)
        );
    }
}
