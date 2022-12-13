use serde_json::Value;
use std::cmp::Ordering;

pub fn part_one(input: &str) -> Option<u32> {
    let input = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .collect::<Vec<Value>>()
        .chunks(2)
        .map(|c| (c[0].clone(), c[1].clone()))
        .collect::<Vec<(Value, Value)>>();

    Some(
        input
            .iter()
            .enumerate()
            .filter(|(_, v)| cmp(&v.0, &v.1).is_lt())
            .map(|(i, _)| i as u32 + 1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .collect::<Vec<Value>>();

    let markers = "[[2]]\n[[6]]"
        .lines()
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .collect::<Vec<Value>>();
    input.extend_from_slice(&markers);
    input.sort_by(cmp);

    Some(
        input
            .iter()
            .enumerate()
            .filter(|(_, v)| markers.contains(v))
            .map(|(i, _)| i as u32 + 1)
            .reduce(|a, b| a * b)
            .unwrap(),
    )
}

fn cmp(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => a.as_u64().cmp(&b.as_u64()),
        (Value::Array(a), Value::Array(b)) => {
            let mut a = a.iter();
            let mut b = b.iter();
            loop {
                return match (a.next(), b.next()) {
                    (Some(a), Some(b)) => match cmp(a, b) {
                        Ordering::Equal => continue, // skip equal values
                        ord => ord,
                    },
                    (None, None) => Ordering::Equal,
                    (None, _) => Ordering::Less,
                    (_, None) => Ordering::Greater,
                };
            }
        }
        (Value::Array(_), Value::Number(_)) => cmp(a, &Value::Array(vec![b.clone()])),
        (Value::Number(_), Value::Array(_)) => cmp(&Value::Array(vec![a.clone()]), b),
        _ => panic!("should never reach this"),
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 13);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
