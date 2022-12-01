
pub fn part_one(input: &str) -> Option<u32> {
    let mut s: Vec<u32> = input
        .split("\n\n")
        .into_iter()
        .map(|lines| {
            lines
                .lines()
                .map(|n| n.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();
    s.sort();
    Some(s[s.len()-1])
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut s: Vec<u32> = input
        .split("\n\n")
        .into_iter()
        .map(|lines| {
            lines
                .lines()
                .map(|n| n.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();
    s.sort();
    Some(s[s.len() - 3..s.len()].iter().sum())
}

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
