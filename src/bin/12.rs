pub fn part_one(input: &str) -> Option<u32> {
    let mut s: (usize, usize) = (0, 0);
    let mut t: (usize, usize) = (0, 0);
    let h = input.lines().count();
    let w = input.lines().next().unwrap().len();
    let mut map: Vec<u32> = vec![0; w * h];
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let v = match c {
                'S' => {
                    s = (x, y);
                    'a' as u32
                }
                'E' => {
                    t = (x, y);
                    'z' as u32
                }
                v => v as u32,
            };
            map[from_coord((x, y), w)] = v;
        }
    }

    let mut dist: Vec<u32> = vec![u32::MAX - 1; map.len()];
    let mut prev: Vec<Option<(usize, usize)>> = vec![None; map.len()];
    let mut q: Vec<(usize, usize)> = vec![];
    for y in 0..h {
        for x in 0..w {
            q.push((x, y));
        }
    }
    dist[from_coord(s, w)] = 0;

    let dn: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
    while q.len() > 0 {
        let u = q.remove(
            q.iter()
                .enumerate()
                .min_by(|(_, &a), (_, &b)| dist[from_coord(a, w)].cmp(&dist[from_coord(b, w)]))
                .map(|(i, _)| i)
                .unwrap(),
        );
        if u == t {
            break;
        }

        let c = map[from_coord(u, w)];
        for v in dn
            .iter()
            .map(|n| ((n.0 + u.0 as i32) as usize, (n.1 + u.1 as i32) as usize))
            .filter(|n| q.contains(n))
            .filter(|&n| map[from_coord(n, w)] as i32 - c as i32 <= 1)
        {
            let z = dist[from_coord(u, w)];
            let alt = z + 1;
            if alt < dist[from_coord(v, w)] {
                dist[from_coord(v, w)] = alt;
                prev[from_coord(v, w)] = Some(u);
            }
        }
    }

    let mut s: Vec<(usize, usize)> = vec![];
    let mut u = t;
    loop {
        match prev[from_coord(u, w)] {
            Some(n) => {
                s.push(u);
                u = n;
            }
            None => break,
        }
    }

    Some(s.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut s: (usize, usize) = (0, 0);
    let h = input.lines().count();
    let w = input.lines().next().unwrap().len();
    let mut map: Vec<u32> = vec![0; w * h];
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let v = match c {
                'S' => 'a' as u32,
                'E' => {
                    s = (x, y);
                    'z' as u32
                }
                v => v as u32,
            };
            map[from_coord((x, y), w)] = v;
        }
    }

    let mut dist: Vec<u32> = vec![u32::MAX - 1; map.len()];
    let mut prev: Vec<Option<(usize, usize)>> = vec![None; map.len()];
    let mut q: Vec<(usize, usize)> = vec![];
    for y in 0..h {
        for x in 0..w {
            q.push((x, y));
        }
    }
    dist[from_coord(s, w)] = 0;

    let dn: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
    let mut t = (0, 0);
    while q.len() > 0 {
        let u = q.remove(
            q.iter()
                .enumerate()
                .min_by(|(_, &a), (_, &b)| dist[from_coord(a, w)].cmp(&dist[from_coord(b, w)]))
                .map(|(i, _)| i)
                .unwrap(),
        );
        if map[from_coord(u, w)] == 'a' as u32 {
            t = u;
            break;
        }

        let c = map[from_coord(u, w)];
        for v in dn
            .iter()
            .map(|n| ((n.0 + u.0 as i32) as usize, (n.1 + u.1 as i32) as usize))
            .filter(|n| q.contains(n))
            .filter(|&n| c as i32 - map[from_coord(n, w)] as i32 <= 1)
        {
            let z = dist[from_coord(u, w)];
            let alt = z + 1;
            if alt < dist[from_coord(v, w)] {
                dist[from_coord(v, w)] = alt;
                prev[from_coord(v, w)] = Some(u);
            }
        }
    }

    let mut s: Vec<(usize, usize)> = vec![];
    let mut u = t;
    loop {
        match prev[from_coord(u, w)] {
            Some(n) => {
                s.push(u);
                u = n;
            }
            None => break,
        }
    }

    Some(s.len() as u32)
}

fn from_coord(p: (usize, usize), w: usize) -> usize {
    p.0 + p.1 * w
}

fn main() {
    let input = &aoc::read_file("inputs", 12);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
