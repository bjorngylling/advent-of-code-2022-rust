use regex::{Match, Regex};

fn to_usize(s: Option<Match>) -> usize {
    s.unwrap().as_str().parse::<usize>().unwrap()
}

pub fn part_one(input: &str) -> Option<String> {
    let mut stacks: Vec<Vec<char>> = vec![vec![]; (input.lines().next().unwrap().len() + 1) / 4];
    let mut lns = input.lines().into_iter();
    // parse stack configuration
    loop {
        let l = lns.next().unwrap();
        if l.chars().nth(1).unwrap() == '1' {
            break;
        }
        for i in 0..stacks.len() {
            let c = l.chars().nth(i * 4 + 1).unwrap();
            if c != ' ' {
                stacks[i].push(c)
            }
        }
    }
    lns.next(); // skip empty line

    stacks.iter_mut().for_each(|s| s.reverse());

    // perform crane operations
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    loop {
        let l = lns.next();
        if matches!(l, None) {
            break;
        }
        let cap = re.captures(l.unwrap()).unwrap();
        for _ in 0..to_usize(cap.get(1)) {
            let c = stacks[to_usize(cap.get(2)) - 1].pop().unwrap();
            stacks[to_usize(cap.get(3)) - 1].push(c);
        }
    }
    let s = stacks.iter().map(|s| s.last().unwrap()).collect();
    Some(s)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut stacks: Vec<Vec<char>> = vec![vec![]; (input.lines().next().unwrap().len() + 1) / 4];
    let mut lns = input.lines().into_iter();
    // parse stack configuration
    loop {
        let l = lns.next().unwrap();
        if l.chars().nth(1).unwrap() == '1' {
            break;
        }
        for i in 0..stacks.len() {
            let c = l.chars().nth(i * 4 + 1).unwrap();
            if c != ' ' {
                stacks[i].push(c)
            }
        }
    }
    lns.next(); // skip empty line

    stacks.iter_mut().for_each(|s| s.reverse());

    // perform crane operations
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    loop {
        let l = lns.next();
        if matches!(l, None) {
            break;
        }
        let cap = re.captures(l.unwrap()).unwrap();
        let mut t: Vec<char> = vec![];
        for _ in 0..to_usize(cap.get(1)) {
            t.push(stacks[to_usize(cap.get(2)) - 1].pop().unwrap());
        }
        t.reverse();
        stacks[to_usize(cap.get(3)) - 1].extend(t);
    }
    let s = stacks.iter().map(|s| s.last().unwrap()).collect();
    Some(s)
}
fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
