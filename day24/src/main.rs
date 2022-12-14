use std::fmt::{Display, Error, Formatter};
use rayon::prelude::*;

const INPUT_DATA: &str = include_str!("input.txt");
#[derive(Debug, Clone, Copy)]
enum Variable {
    W,
    X,
    Y,
    Z,
    Var(i64),
}

struct State {
    input: Vec<u32>,
    i: usize,
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "State: w:{}, x:{}, y:{}, z:{}",
            self.w, self.x, self.y, self.z
        )
    }
}

impl State {
    fn is_valid(&self) -> bool {
        self.z == 0
    }
    fn new(input: u64) -> Option<Self> {
        let input = input
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        if input.contains(&0) {
            return None;
        }

        Some(Self {
            input: input,
            i: 0,
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        })
    }
    fn apply_all(&mut self, ops: &Vec<Op>) {
        for op in ops {
            self.apply(op);
        }
    }
    fn apply(&mut self, op: &Op) {
        match op {
            Op::Inp(v) => {
                let val = self.next();
                self.set_value(v, val)
            }
            Op::Add(v, y) => self.set_value(v, self.get_value(v) + self.get_value(y)),
            Op::Mul(v, y) => self.set_value(v, self.get_value(v) * self.get_value(y)),
            Op::Div(v, y) => self.set_value(v, self.get_value(v) / self.get_value(y)),
            Op::Mod(v, y) => self.set_value(v, self.get_value(v) % self.get_value(y)),
            Op::Eql(v, y) => self.set_value(
                v,
                if self.get_value(v) == self.get_value(y) {
                    1
                } else {
                    0
                },
            ),
        }
    }

    fn next(&mut self) -> i64 {
        let last_index = self.i;
        self.i = self.i + 1;
        self.input[last_index].try_into().unwrap()
    }
    fn get_value(&self, v: &Variable) -> i64 {
        match v {
            Variable::Var(val) => *val,
            Variable::W => self.w,
            Variable::X => self.x,
            Variable::Y => self.y,
            Variable::Z => self.z,
        }
    }

    fn set_value(&mut self, v: &Variable, value: i64) {
        match v {
            Variable::W => self.w = value,
            Variable::X => self.x = value,
            Variable::Y => self.y = value,
            Variable::Z => self.z = value,
            _ => panic!(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum Op {
    Inp(Variable),
    Add(Variable, Variable),
    Mul(Variable, Variable),
    Div(Variable, Variable),
    Mod(Variable, Variable),
    Eql(Variable, Variable),
}
fn parse_var(input: &str) -> Variable {
    match input {
        "w" => Variable::W,
        "x" => Variable::X,
        "y" => Variable::Y,
        "z" => Variable::Z,
        _ => Variable::Var(input.parse().unwrap()),
    }
}
fn parse(input: &str) -> Vec<Op> {
    input
        .split("\n")
        .into_iter()
        .map(|line| {
            let (op, rem) = line.split_once(" ").unwrap();
            let operands = rem.split(" ").map(parse_var).collect::<Vec<_>>();
            match op {
                "inp" => Op::Inp(parse_var(rem)),
                "add" => Op::Add(operands[0], operands[1]),
                "mul" => Op::Mul(operands[0], operands[1]),
                "div" => Op::Div(operands[0], operands[1]),
                "mod" => Op::Mod(operands[0], operands[1]),
                "eql" => Op::Eql(operands[0], operands[1]),
                _ => panic!(),
            }
        })
        .collect()
}
fn main() {
    /*
    let ops = parse(TEST_DATA);

    for i in 0..10 {
        let maybe_state = State::new(i);
        if let Some(mut state) = maybe_state {
            state.apply_all(&ops);

            println!(" {}", &state);
        }
    }
 */
let max: u64 = 99999999999999;
let ops = parse(INPUT_DATA);
    /*
    for i in 0..max {
        let current = max - i;
        let maybe_state = State::new(current);
        if let Some(mut state) = maybe_state {
            state.apply_all(&ops);
            if state.is_valid() {
                println!("input {}, state {}", current, &state);
                return;
            }
        }
        println!("It's not {}", current);
    }
    */
    let vec = (0..(max/10)).collect::<Vec<_>>();

    let result = vec.par_iter().find_last(|&i| {
        let current = max - *i;
        let maybe_state = State::new(current);
        if let Some(mut state) = maybe_state {
            state.apply_all(&ops);
            if state.is_valid() {
                println!("input {}, state {}", current, &state);
                return true;
            }
            
        }
        false
  });
  //vec.par_sort_unstable();

  println!("result {:?}", result);
    
}
