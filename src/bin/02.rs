use regex::Regex;

advent_of_code::solution!(2);

const BAG: (u32, u32, u32) = (12, 13, 14); // (R, G, B)

// FIXME: This solution is slow! I think the regexes need to be precompiled!
// static GAME_ID: Regex = Regex::new(r"Game (\d+)").unwrap();

fn check_game(line: &str) -> Option<u32> {
    let game_id: u32 = Regex::new(r"Game (\d+)")
        .unwrap()
        .captures(line)?
        .get(1)?
        .as_str()
        .parse()
        .ok()?;
    Regex::new(r"\d[^:;]+")
        .unwrap()
        .find_iter(line)
        .all(|m| is_possible_draw(m.as_str()))
        .then_some(game_id)
}

fn is_possible_draw(draw: &str) -> bool {
    Regex::new(r"(\d+) (\w+)")
        .unwrap()
        .captures_iter(draw)
        .filter_map(|c| Some((c.get(1)?.as_str().parse::<u32>().ok()?, c.get(2)?.as_str())))
        .all(|(n, c)| match c {
            "red" => n <= BAG.0,
            "blue" => n <= BAG.1,
            "green" => n <= BAG.2,
            _ => panic!("Invalid colour"),
        })
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().filter_map(check_game).sum())
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
