advent_of_code::solution!(1);

fn process_line(line: &str) -> Option<u32> {
    let digits: Vec<u32> = line.chars().filter_map(|c| char::to_digit(c, 10)).collect();
    Some(10 * digits.first()? + digits.last()?)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().filter_map(process_line).sum())
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
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
