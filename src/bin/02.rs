use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut m: HashMap<&str, u32> = HashMap::new();
    m.insert("A X", 4);
    m.insert("A Y", 8);
    m.insert("A Z", 3);
    m.insert("B X", 1);
    m.insert("B Y", 5);
    m.insert("B Z", 9);
    m.insert("C X", 7);
    m.insert("C Y", 2);
    m.insert("C Z", 6);
    Some(input.lines().map(|l| m[l]).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut m: HashMap<&str, u32> = HashMap::new();
    m.insert("A X", 3);
    m.insert("A Y", 4);
    m.insert("A Z", 8);
    m.insert("B X", 1);
    m.insert("B Y", 5);
    m.insert("B Z", 9);
    m.insert("C X", 2);
    m.insert("C Y", 6);
    m.insert("C Z", 7);
    Some(input.lines().map(|l| m[l]).sum())
}

fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
