pub fn part_one(input: &str) -> Option<u32> {
    let w = 550;
    let h = 300;
    let mut world = vec![0_u8; w * h];
    for l in input.lines() {
        let points = l
            .split("->")
            .map(|s| s.trim())
            .map(|s| {
                let mut p = s.split(",");
                (
                    p.next().unwrap().parse::<i32>().unwrap(),
                    p.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .collect::<Vec<(i32, i32)>>();
        for p in points.windows(2) {
            let mut xr = [p[0].0, p[1].0];
            xr.sort();
            let mut yr = [p[0].1, p[1].1];
            yr.sort();
            for x in xr[0]..=xr[1] {
                for y in yr[0]..=yr[1] {
                    world[from_coord((x as usize, y as usize), w)] = 1;
                }
            }
        }
    }

    'OUTER: loop {
        let mut s = (500, 0);
        loop {
            if s.1 + 1 >= h {
                break 'OUTER;
            }
            match (
                world[from_coord((s.0 - 1, s.1 + 1), w)],
                world[from_coord((s.0, s.1 + 1), w)],
                world[from_coord((s.0 + 1, s.1 + 1), w)],
            ) {
                (_, 0, _) => s.1 += 1,
                (0, 1..=2, _) => s = (s.0 - 1, s.1 + 1),
                (_, 1..=2, 0) => s = (s.0 + 1, s.1 + 1),
                (1..=2, 1..=2, 1..=2) => {
                    world[from_coord(s, w)] = 2;
                    break;
                }
                (a, b, c) => panic!("unexpected world state {},{},{}", a, b, c),
            }
        }
    }
    Some(world.iter().filter(|&&i| i == 2_u8).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let w = 950;
    let h = 300;
    let mut max_y = 0;
    let mut world = vec![0_u8; w * h];
    for l in input.lines() {
        let points = l
            .split("->")
            .map(|s| s.trim())
            .map(|s| {
                let mut p = s.split(",");
                (
                    p.next().unwrap().parse::<i32>().unwrap(),
                    p.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .collect::<Vec<(i32, i32)>>();
        for p in points.windows(2) {
            let mut xr = [p[0].0, p[1].0];
            xr.sort();
            let mut yr = [p[0].1, p[1].1];
            yr.sort();
            for x in xr[0]..=xr[1] {
                for y in yr[0]..=yr[1] {
                    world[from_coord((x as usize, y as usize), w)] = 1;
                    max_y = max_y.max(y);
                }
            }
        }
    }
    max_y += 2;
    for x in 0..=w {
        world[from_coord((x, max_y as usize), w)] = 1;
    }

    loop {
        if world[from_coord((500, 0), w)] == 2 {
            break;
        }
        let mut s = (500, 0);
        loop {
            match (
                world[from_coord((s.0 - 1, s.1 + 1), w)],
                world[from_coord((s.0, s.1 + 1), w)],
                world[from_coord((s.0 + 1, s.1 + 1), w)],
            ) {
                (_, 0, _) => s.1 += 1,
                (0, 1..=2, _) => s = (s.0 - 1, s.1 + 1),
                (_, 1..=2, 0) => s = (s.0 + 1, s.1 + 1),
                (1..=2, 1..=2, 1..=2) => {
                    world[from_coord(s, w)] = 2;
                    break;
                }
                (a, b, c) => panic!("unexpected world state {},{},{}", a, b, c),
            }
        }
    }
    Some(world.iter().filter(|&&i| i == 2_u8).count() as u32)
}

fn from_coord(p: (usize, usize), w: usize) -> usize {
    return p.0 + p.1 * w;
}

fn main() {
    let input = &aoc::read_file("inputs", 14);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
