use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let l = input.chars().collect::<Vec<char>>();
    for i in 3..=l.len() {
        if HashSet::<&char>::from_iter(l[(i-3)..=i].iter()).len() == 4 {
            return Some(i as u32 + 1);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let l = input.chars().collect::<Vec<char>>();
    for i in 13..=l.len() {
        if HashSet::<&char>::from_iter(l[(i-13)..=i].iter()).len() == 14 {
            return Some(i as u32 + 1);
        }
    }
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 6);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
        assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        assert_eq!(part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));
    }
}
