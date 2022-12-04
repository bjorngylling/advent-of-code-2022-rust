use regex::{Match, Regex};

fn to_u32(s: Option<Match>) -> u32 {
    s.unwrap().as_str().parse::<u32>().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    Some(
        input
            .lines()
            .map(|l| {
                let d = re.captures(l).unwrap();
                (
                    (to_u32(d.get(1)))..=to_u32(d.get(2)),
                    (to_u32(d.get(3)))..=to_u32(d.get(4)),
                )
            })
            .filter(|p| {
                !(p.0.clone().any(|i| !p.1.contains(&i)) && p.1.clone().any(|i| !p.0.contains(&i)))
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    Some(
        input
            .lines()
            .map(|l| {
                let d = re.captures(l).unwrap();
                (
                    (to_u32(d.get(1)))..=to_u32(d.get(2)),
                    (to_u32(d.get(3)))..=to_u32(d.get(4)),
                )
            })
            .filter(|p| p.0.clone().any(|i| p.1.contains(&i)) || p.1.clone().any(|i| p.0.contains(&i)))
            .count() as u32,
    )
}

fn main() {
    let input = &aoc::read_file("inputs", 4);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
