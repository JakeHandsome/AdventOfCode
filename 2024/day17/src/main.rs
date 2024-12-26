use common::*;
use num::{pow, FromPrimitive};
use num_derive::FromPrimitive;

fn main() {
    let input = read_input_file_for_project_as_string!();
    {
        let _timer = Timer::new("Part 1");
        println!("Part1: {}", part1(&input).unwrap());
    }
    {
        let _timer = Timer::new("Part 2");
        println!("Part2: {}", part2(&input).unwrap());
    }
}

#[derive(FromPrimitive)]
enum OpCode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

#[derive(Debug)]
struct Computer {
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
    program: Vec<usize>,
    out: Vec<usize>,
}

impl Computer {
    fn new(input: &str) -> Self {
        let mut iter = input.lines();
        let a = iter.next().unwrap().split(":").nth(1).unwrap().trim().parse().unwrap();
        let b = iter.next().unwrap().split(":").nth(1).unwrap().trim().parse().unwrap();
        let c = iter.next().unwrap().split(":").nth(1).unwrap().trim().parse().unwrap();
        let _ = iter.next();
        let program = iter
            .next()
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect_vec();

        Self {
            a,
            b,
            c,
            ip: 0,
            program,
            out: vec![],
        }
    }
    fn run(&mut self) -> anyhow::Result<()> {
        loop {
            self.step()?;
        }
    }
    #[inline]
    fn step(&mut self) -> anyhow::Result<()> {
        //println!("{:?}", &self);
        let instruction = self.fetch()?;
        let literal_op = self.fetch()?;
        let combo_op = match literal_op {
            0..=3 => literal_op,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        };
        match OpCode::from_usize(instruction) {
            Some(OpCode::Adv) => self.a /= 1 << combo_op,
            Some(OpCode::Bxl) => self.b ^= literal_op,
            Some(OpCode::Bst) => self.b = combo_op % 8,
            Some(OpCode::Jnz) => {
                if self.a != 0 {
                    self.ip = literal_op
                }
            }
            Some(OpCode::Bxc) => self.b ^= self.c,
            Some(OpCode::Out) => self.out.push(combo_op % 8),
            Some(OpCode::Bdv) => self.b = self.a / (1 << combo_op),
            Some(OpCode::Cdv) => self.c = self.a / (1 << combo_op),
            None => unreachable!(),
        }
        Ok(())
    }
    fn fetch(&mut self) -> anyhow::Result<usize> {
        let a = self
            .program
            .get(self.ip)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("Halt"))?;
        self.ip += 1;
        Ok(a)
    }
    fn reset(&mut self, a: usize) {
        self.a = a;
        self.b = 0;
        self.c = 0;
        self.ip = 0;
        self.out = vec![];
    }
}

fn part1(input: &str) -> anyhow::Result<String> {
    let mut c = Computer::new(input);
    let _ = c.run();
    Ok(c.out.into_iter().join(","))
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut c = Computer::new(input);
    let output_len = c.program.len();
    let mut a = pow(8, output_len - 1);
    let mut digit = output_len - 2;
    loop {
        c.reset(a);
        loop {
            if c.step().is_err() {
                if c.out == c.program {
                    return Ok(a);
                }
                if c.out[digit..] == c.program[digit..] {
                    digit -= 1;
                }
                break;
            }
        }
        a += pow(8, digit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), "4,6,3,5,6,3,5,2,1,0");
    }
    const SAMPLE2: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE2).unwrap(), 117440);
    }
}
