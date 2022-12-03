pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let spl = l.split_at(l.len() / 2);
                spl.0.chars().find(|c| spl.1.contains(*c))
            })
            .map(priority)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|group| {
                group[0].chars().find(|c| group[1].contains(*c) && group[2].contains(*c))
            })
            .map(priority)
            .sum(),
    )
}

fn priority(c: Option<char>) -> u32 {
    if let Some(c) = c {
        if c.is_ascii_uppercase() {
            c as u32 - 38
        } else {
            c as u32 - 96
        }
    } else {
        0
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(priority(Some('a')), 1);
        assert_eq!(priority(Some('z')), 26);
        assert_eq!(priority(Some('A')), 27);
        assert_eq!(priority(Some('Z')), 52);
    }

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
