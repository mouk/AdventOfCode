//const TEST_DATA: &str = include_str!("test.txt");
const INPUT_DATA: &str = include_str!("input.txt");
use std::{str::FromStr, collections::HashSet};

#[derive(Debug, PartialEq,Clone,Copy)]
enum Op{
    Acc,
    Jmp,
    Nop
}

impl FromStr for Op {

    type Err = ();

    fn from_str(input: &str) -> Result<Op, Self::Err> {
        match input {
            "acc"  => Ok(Op::Acc),
            "nop"  => Ok(Op::Nop),
            "jmp"  => Ok(Op::Jmp),
            _      => Err(()),
        }
    }
}


struct Instr{
    op: Op,
    arg: i32
}

struct Program{
    instructions: Vec<Instr>
}
impl Program{
    fn new(input: &str) -> Self{
        let lines = input
        .split('\n')
        .map(|l| {
            let (op, arg) = l.split_once(" ").unwrap();
            let op = Op::from_str(op).unwrap();
            let arg = arg.parse().unwrap();
            Instr{op:op,arg:arg}
        })
        .collect::<_>();
        Self{
            instructions: lines
        }
    }
}

impl Program {
    fn run(&self) -> i32{
        let mut acc:i32 = 0;
        let mut offset:usize = 0;
        let mut visited = HashSet::new();
        loop {
            if visited.contains(&offset) {
                        break;
            }
            visited.insert(offset.clone());
            let curr = &self.instructions[offset];
            match curr.op {
                Op::Acc => {
                    acc += curr.arg;
                    offset += 1
                },
                Op::Jmp => {
                    offset = ( curr.arg + (offset as i32)) as usize
                },
                _ => {
                    offset += 1
                },
            }
        }
        acc
    }

    fn fix_and_run(&self) -> i32{
        

        let inst_count = self.instructions.len();

        for fixed_index in 0..self.instructions.len(){
            if self.instructions[fixed_index].op == Op::Acc{
                continue;
            }


            let mut acc:i32 = 0;
            let mut offset:usize = 0;
            let mut visited = HashSet::new();

            loop {
                if offset == inst_count{
                    return acc;
                }
                if visited.contains(&offset) {
                            break;
                }
                visited.insert(offset.clone());
                let curr = &self.instructions[offset];
                let arg = curr.arg;
                let op = if fixed_index == offset{
                    if curr.op == Op::Jmp {
                        Op::Nop
                    }else{
                        Op::Jmp
                    }

                }else{
                    curr.op
                };
                
                match op {
                    Op::Acc => {
                        acc += arg;
                        offset += 1
                    },
                    Op::Jmp => {
                        offset = ( arg + (offset as i32)) as usize
                    },
                    _ => {
                        offset += 1
                    },
                }
            }
        }
        unreachable!()
    }
}

fn main() {
    let p = Program::new(INPUT_DATA);
    let result = p.run();
    println!("Part 1: {}", result);
    let result = p.fix_and_run();
    println!("Part 2: {}", result);
}
