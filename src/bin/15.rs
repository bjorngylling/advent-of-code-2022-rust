use std::collections::HashSet;

fn p1(input: &str, check_y: i32) -> Option<u32> {
    let sensors = input
        .lines()
        .map(|l| {
            if let [sensor, beacon] = l
                .replace("Sensor at x=", "")
                .replace(" y=", "")
                .replace(" closest beacon is at x=", "")
                .split(":")
                .collect::<Vec<_>>()[0..=1]
            {
                (
                    {
                        let mut s = sensor.split(",").map(|s| s.parse::<i32>().unwrap());
                        (s.next().unwrap(), s.next().unwrap())
                    },
                    {
                        let mut b = beacon.split(",").map(|b| b.parse::<i32>().unwrap());
                        (b.next().unwrap(), b.next().unwrap())
                    },
                )
            } else {
                panic!("unexpected input: {}", l);
            }
        })
        .collect::<Vec<_>>();
    let mut intervals = sensors
        .iter()
        .filter_map(|s| {
            let radius = (s.0 .0 - s.1 .0).abs() + (s.0 .1 - s.1 .1).abs();
            let dr = (s.0 .1 - check_y).abs();
            if dr <= radius {
                Some((s.0 .0 - (radius - dr), s.0 .0 + (radius - dr)))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    intervals.sort();

    let disjoint = intervals
        .into_iter()
        .fold(vec![], |mut accum: Vec<(i32, i32)>, interval| {
            if accum.is_empty() {
                accum.push(interval);
            } else {
                let prev = {
                    let idx = accum.len() - 1;
                    accum.get_mut(idx).unwrap()
                };
                if interval.0 - 1 <= prev.1 {
                    prev.1 = prev.1.max(interval.1);
                } else {
                    accum.push(interval);
                }
            }
            accum
        });

    let mut count = disjoint.iter().fold(0, |a, i| a + (i.1 - i.0 + 1));
    let mut counted_beacons: HashSet<i32> = HashSet::new();
    for s in sensors {
        if s.1 .1 == check_y && counted_beacons.insert(s.1 .0) {
            count -= 1;
        }
    }
    Some(count as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    p1(input, 2000000)
}

fn p2(input: &str, max_y: i32) -> Option<u64> {
    let sensors = input
        .lines()
        .map(|l| {
            if let [sensor, beacon] = l
                .replace("Sensor at x=", "")
                .replace(" y=", "")
                .replace(" closest beacon is at x=", "")
                .split(":")
                .collect::<Vec<_>>()[0..=1]
            {
                (
                    {
                        let mut s = sensor.split(",").map(|s| s.parse::<i32>().unwrap());
                        (s.next().unwrap(), s.next().unwrap())
                    },
                    {
                        let mut b = beacon.split(",").map(|b| b.parse::<i32>().unwrap());
                        (b.next().unwrap(), b.next().unwrap())
                    },
                )
            } else {
                panic!("unexpected input: {}", l);
            }
        })
        .collect::<Vec<_>>();
    let mut p = (0, 0);
    for check_y in 0..max_y {
        let mut intervals = sensors
            .iter()
            .filter_map(|s| {
                let radius = (s.0 .0 - s.1 .0).abs() + (s.0 .1 - s.1 .1).abs();
                let dr = (s.0 .1 - check_y).abs();
                if dr <= radius {
                    Some((s.0 .0 - (radius - dr), s.0 .0 + (radius - dr)))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        intervals.sort();

        let disjoint =
            intervals
                .into_iter()
                .fold(vec![], |mut accum: Vec<(i32, i32)>, interval| {
                    if accum.is_empty() {
                        accum.push(interval);
                    } else {
                        let prev = {
                            let idx = accum.len() - 1;
                            accum.get_mut(idx).unwrap()
                        };
                        if interval.0 - 1 <= prev.1 {
                            prev.1 = prev.1.max(interval.1);
                        } else {
                            accum.push(interval);
                        }
                    }
                    accum
                });

        if disjoint.len() > 1 {
            return Some((disjoint[0].1 + 1) as u64 * 4_000_000 + check_y as u64);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    p2(input, 4_000_000)
}

fn main() {
    let input = &aoc::read_file("inputs", 15);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 15);
        assert_eq!(p1(&input, 10), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 15);
        assert_eq!(p2(&input, 20), Some(56000011));
    }
}
