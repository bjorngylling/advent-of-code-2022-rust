struct CPU {
    x: i32,
    cycle: i32,
    sig: Vec<i32>,
    disp: Vec<char>,
}

impl CPU {
    fn new() -> Self {
        CPU {
            x: 1,
            cycle: 0,
            sig: vec![],
            disp: vec![],
        }
    }

    fn run(&mut self, instr: &str) {
        let mut i = instr.split_whitespace().collect::<Vec<&str>>();
        if i.len() == 1 {
            i.push("")
        }
        match i[0..=1] {
            ["addx", d] => {
                self.cycle();
                self.cycle();
                self.x += d.parse::<i32>().unwrap();
            }
            _ => self.cycle(),
        }
    }

    fn cycle(&mut self) {
        let spr = self.x - 1..=self.x + 1;
        if spr.contains(&(self.cycle % 40)) {
            self.disp.push('#');
        } else {
            self.disp.push('.');
        }
        self.cycle += 1;
        if (self.cycle - 20) % 40 == 0 {
            self.sig.push(self.cycle * self.x);
        }
        if self.cycle % 40 == 0 {
            self.disp.push('\n');
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut cpu = CPU::new();
    for l in input.lines() {
        cpu.run(l);
    }
    Some(cpu.sig.iter().sum())
}

fn part_two_output(input: &str) -> String {
    let mut cpu = CPU::new();
    for l in input.lines() {
        cpu.run(l);
    }
    cpu.disp.into_iter().collect()
}

pub fn part_two(input: &str) -> Option<i32> {
    print!("{}", part_two_output(input));
    Some(1)
}

fn main() {
    let input = &aoc::read_file("inputs", 10);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(
            part_two_output(&input),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
    }
}
