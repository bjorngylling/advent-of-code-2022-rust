use std::collections::VecDeque;

struct Monkey {
    items: VecDeque<u64>,
    op: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> usize>,
    cnt: u64,
    t_div: u64,
}

impl Monkey {
    fn new() -> Self {
        Self {
            items: VecDeque::new(),
            op: Box::new(|a| a),
            test: Box::new(|_| 0),
            cnt: 0,
            t_div: 1,
        }
    }

    fn from(s: &str) -> Self {
        let chunk: Vec<&str> = s.lines().collect();
        let mut m = Monkey::new();
        m.items = chunk[1]
            .trim_start_matches("  Starting items: ")
            .split(",")
            .map(|i| i.trim().parse::<u64>().unwrap())
            .collect();

        m.op = match chunk[2].split_whitespace().collect::<Vec<&str>>()[4..6] {
            ["*", "old"] => Box::new(|a| a * a),
            ["*", d] => {
                let i = d.parse::<u64>().unwrap();
                Box::new(move |a| a * i)
            }
            ["+", d] => {
                let i = d.parse::<u64>().unwrap();
                Box::new(move |a| a + i)
            }
            _ => Box::new(|a| a),
        };
        let div = chunk[3]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let t = chunk[4]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let f = chunk[5]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        m.test = Box::new(move |a| if a % div == 0 { t } else { f });
        m.t_div = div;
        m
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input: Vec<&str> = input.split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = vec![];
    for m in input {
        monkeys.push(Monkey::from(m));
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let item = (monkeys[i].op)(item) / 3;
                let j = (monkeys[i].test)(item);
                monkeys[j].items.push_back(item);
                monkeys[i].cnt += 1;
            }
        }
    }

    let mut l: Vec<u64> = monkeys.iter().map(|m| m.cnt).collect();
    l.sort();
    l.reverse();

    Some(l[0] * l[1])
}

pub fn part_two(input: &str) -> Option<u64> {
    let input: Vec<&str> = input.split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = vec![];
    let mut lcm = 1;
    for c in input {
        let m = Monkey::from(c);
        lcm = num_integer::lcm(lcm, m.t_div);
        monkeys.push(m);
    }

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let item = (monkeys[i].op)(item);
                let j = (monkeys[i].test)(item);
                monkeys[j].items.push_back(item%lcm);
                monkeys[i].cnt += 1;
            }
        }
    }

    let mut l: Vec<u64> = monkeys.iter().map(|m| m.cnt).collect();
    l.sort();
    l.reverse();

    Some(l[0] * l[1])
}
fn main() {
    let input = &aoc::read_file("inputs", 11);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
