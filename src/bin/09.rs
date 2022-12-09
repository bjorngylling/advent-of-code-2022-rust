use std::collections::HashSet;

enum Move {
    U(usize),
    R(usize),
    D(usize),
    L(usize),
}

pub fn part_one(input: &str) -> Option<u32> {
    let instr: Vec<Move> = input
        .lines()
        .map(|l| match l.split_whitespace().collect::<Vec<_>>()[0..=1] {
            ["R", d] => Move::R(d.parse().unwrap()),
            ["D", d] => Move::D(d.parse().unwrap()),
            ["L", d] => Move::L(d.parse().unwrap()),
            ["U", d] => Move::U(d.parse().unwrap()),
            _ => panic!("unexpected input {}", l),
        })
        .collect();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    for m in instr {
        let steps: Vec<(i32, i32)>;
        match m {
            Move::R(d) => steps = vec![(1, 0); d],
            Move::D(d) => steps = vec![(0, 1); d],
            Move::L(d) => steps = vec![(-1, 0); d],
            Move::U(d) => steps = vec![(0, -1); d],
        }
        for s in steps {
            head.0 += s.0;
            head.1 += s.1;

            // Move the tail if required
            let d = match (head.0 - tail.0, head.1 - tail.1) {
                (-2, y) => (-1, y),
                (2, y) => (1, y),
                (x, -2) => (x, -1),
                (x, 2) => (x, 1),
                _ => (0, 0),
            };
            tail.0 += d.0;
            tail.1 += d.1;
            visited.insert(tail);
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let instr: Vec<Move> = input
        .lines()
        .map(|l| match l.split_whitespace().collect::<Vec<_>>()[0..=1] {
            ["R", d] => Move::R(d.parse().unwrap()),
            ["D", d] => Move::D(d.parse().unwrap()),
            ["L", d] => Move::L(d.parse().unwrap()),
            ["U", d] => Move::U(d.parse().unwrap()),
            _ => panic!("unexpected input {}", l),
        })
        .collect();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut tail = vec![(0, 0); 10];
    for m in instr {
        let steps: Vec<(i32, i32)>;
        match m {
            Move::R(d) => steps = vec![(1, 0); d],
            Move::D(d) => steps = vec![(0, 1); d],
            Move::L(d) => steps = vec![(-1, 0); d],
            Move::U(d) => steps = vec![(0, -1); d],
        }
        for s in steps {
            tail[0].0 += s.0;
            tail[0].1 += s.1;

            // Move the tail if required
            for seg in 1..=9 {
                let d = (tail[seg - 1].0 - tail[seg].0, tail[seg - 1].1 - tail[seg].1);
                if d.0 >= -1 && d.0 <= 1 && d.1 >= -1 && d.1 <= 1 {
                    continue;
                }
                tail[seg].0 += d.0.signum();
                tail[seg].1 += d.1.signum();
            }
            visited.insert(tail[tail.len() - 1]);
        }
    }

    Some(visited.len() as u32)
}

fn main() {
    let input = &aoc::read_file("inputs", 9);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(part_two(&input), Some(36));
    }
}
