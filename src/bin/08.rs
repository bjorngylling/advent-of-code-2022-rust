pub fn part_one(input: &str) -> Option<u32> {
    let w = input.lines().next().unwrap().len();
    let h = input.lines().count();
    let map: Vec<u32> = input
        .replace("\n", "")
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    let mut vis_map: Vec<u32> = vec![0; map.len()];
    for y in 0..h {
        for x in 0..w {
            let mut vis_dirs = 0;
            let v = map[from_coord((x, y), w)];
            for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let mut p = (x, y);
                let mut blocked = false;
                while !blocked {
                    p = apply_delta(p, d);
                    if p.0 >= w || p.1 >= h {
                        break;
                    }
                    if map[from_coord(p, w)] >= v {
                        blocked = true;
                    }
                }
                if !blocked {
                    vis_dirs += 1
                }
            }
            vis_map[from_coord((x, y), w)] = vis_dirs;
        }
    }

    Some(vis_map.iter().filter(|&&c| c > 0).count() as u32)
}

fn apply_delta(p: (usize, usize), d: (i32, i32)) -> (usize, usize) {
    ((p.0 as i32 + d.0) as usize, (p.1 as i32 + d.1) as usize)
}

fn from_coord(p: (usize, usize), w: usize) -> usize {
    return p.0 + p.1 * w;
}

fn to_coord(i: usize, w: usize) -> (usize, usize) {
    return (i % w, i / w);
}

pub fn part_two(input: &str) -> Option<u32> {
    let w = input.lines().next().unwrap().len();
    let h = input.lines().count();
    let map: Vec<u32> = input
        .replace("\n", "")
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    let mut vis_map: Vec<u32> = vec![0; map.len()];
    for y in 0..h {
        for x in 0..w {
            let mut score = 1;
            let v = map[from_coord((x, y), w)];
            for d in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let mut p = (x, y);
                let mut blocked = false;
                let mut dist = 0;
                while !blocked {
                    p = apply_delta(p, d);
                    if p.0 >= w || p.1 >= h {
                        break;
                    }
                    dist += 1;
                    if map[from_coord(p, w)] >= v {
                        blocked = true;
                    }
                }
                score *= dist;
            }
            vis_map[from_coord((x, y), w)] = score;
        }
    }
    Some(*vis_map.iter().max().unwrap())
}

fn main() {
    let input = &aoc::read_file("inputs", 8);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord() {
        assert_eq!(45, from_coord((5, 4), 10));
        assert_eq!((5, 4), to_coord(45, 10));
    }

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
