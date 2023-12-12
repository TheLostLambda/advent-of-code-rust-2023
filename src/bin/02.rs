use once_cell::sync::Lazy;
use regex::Regex;

advent_of_code::solution!(2);

const BAG: [u32; 3] = [12, 13, 14]; // (R, G, B)

// OPTIMISATION: Again, adding the Lazy leads to a massive ~1000x speedup!
static DRAW: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d[^:;]+").unwrap());
static COLOR: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) (\w+)").unwrap());

fn check_game(line: &str) -> Option<u32> {
    static GAME_ID: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game (\d+)").unwrap());
    let game_id: u32 = GAME_ID.captures(line)?.get(1)?.as_str().parse().ok()?;
    DRAW.find_iter(line)
        .all(|m| is_possible_draw(m.as_str()))
        .then_some(game_id)
}

fn color_index(color: &str) -> usize {
    match color {
        "red" => 0,
        "green" => 1,
        "blue" => 2,
        _ => panic!(),
    }
}

fn is_possible_draw(draw: &str) -> bool {
    COLOR
        .captures_iter(draw)
        .filter_map(|c| Some((c.get(1)?.as_str().parse::<u32>().ok()?, c.get(2)?.as_str())))
        .all(|(n, c)| n <= BAG[color_index(c)])
}

fn min_possible_draw(draw: &str) -> [u32; 3] {
    let mut min_bag = [0, 0, 0];
    COLOR
        .captures_iter(draw)
        .filter_map(|c| Some((c.get(1)?.as_str().parse::<u32>().ok()?, c.get(2)?.as_str())))
        .for_each(|(n, c)| {
            let i = color_index(c);
            min_bag[i] = min_bag[i].max(n);
        });
    min_bag
}

fn min_possible_bag_power(line: &str) -> Option<u32> {
    Some(
        DRAW.find_iter(line)
            .map(|m| min_possible_draw(m.as_str()))
            .reduce(|[ra, ga, ba], [re, ge, be]| [ra.max(re), ga.max(ge), ba.max(be)])?
            .iter()
            .product(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().filter_map(check_game).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().filter_map(min_possible_bag_power).sum())
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
        assert_eq!(result, Some(2286));
    }
}
