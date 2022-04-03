//const TEST_DATA: &str = include_str!("test.txt");
const INPUT_DATA: &str = include_str!("input.txt");
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Acc,
    Jmp,
    Nop,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(input: &str) -> Result<Op, Self::Err> {
        match input {
            "acc" => Ok(Op::Acc),
            "nop" => Ok(Op::Nop),
            "jmp" => Ok(Op::Jmp),
            _ => Err(()),
        }
    }
}

struct Instr {
    op: Op,
    arg: i32,
}

struct Program {
    instructions: Vec<Instr>,
}
struct RunResult{
    acc:i32,
    terminated: bool
}
impl Program {
    fn new(input: &str) -> Self {
        let lines = input
            .split('\n')
            .map(|l| {
                let (op, arg) = l.split_once(" ").unwrap();
                let op = Op::from_str(op).unwrap();
                let arg = arg.parse().unwrap();
                Instr { op: op, arg: arg }
            })
            .collect::<_>();
        Self {
            instructions: lines,
        }
    }
}

impl Program {
    fn run(&self) -> RunResult {
        let mut acc: i32 = 0;
        let mut offset: usize = 0;
        let mut visited = HashSet::new();
        loop {
            
            if offset == self.instructions.len() {
                return RunResult{
                    acc:acc,
                    terminated: true
                }
            }

            if visited.contains(&offset) {
                return RunResult{
                    acc:acc,
                    terminated: false
                }
            }
            visited.insert(offset.clone());
            let curr = &self.instructions[offset];
            match curr.op {
                Op::Acc => {
                    acc += curr.arg;
                    offset += 1
                }
                Op::Jmp => offset = (curr.arg + (offset as i32)) as usize,
                _ => offset += 1,
            }
        }
    }

    fn fix_and_run(&mut self) -> i32 {
        for fixed_index in 0..self.instructions.len() {
            if self.instructions[fixed_index].op == Op::Acc {
                continue;
            }
            let backup = self.instructions[fixed_index].op;

            self.instructions[fixed_index].op =  if  self.instructions[fixed_index].op == Op::Jmp {
                Op::Nop
            } else {
                Op::Jmp
            };
            let result = self.run();
            self.instructions[fixed_index].op  = backup;
            
            if result.terminated {
                return result.acc
            }
        }
        unreachable!()
    }
}

fn main() {
    let mut p = Program::new(INPUT_DATA);
    let result = p.run();
    println!("Part 1: {}", result.acc);
    let result = p.fix_and_run();
    println!("Part 2: {}", result);
}
