use std::collections::VecDeque;

#[derive(Debug)]
enum Op { Add, Multiply}

#[derive(Debug)]
enum OpTarget { Num(usize), Old}
#[derive(Debug)]
struct Monkey{
    items_inspected: usize,
    items: VecDeque<usize>,
    op:  (Op, OpTarget),
    test: usize,
    true_target: usize,
    false_target: usize
}

impl Monkey {
    fn vec_from_text(input: &str) -> Vec<Self>{
        input.split("\n\n").map(Monkey::from_text).collect()
    }

    fn from_text(input: &str) ->Self{
        let mut lines = input.split('\n');
        lines.next();

        let (_,starting) = lines.next().unwrap().split_once(": ").unwrap();
        let  items = starting.split(", ").map(|n| n.parse().unwrap()).collect();

        let (_,op) = lines.next().unwrap().split_once("= old ").unwrap();
        let (sign, num) = op.split_once(" ").unwrap();
        let op = match sign {
            "*" => Op::Multiply,
            _ =>  Op::Add
        };
        let op_target = match num {
            "old" => OpTarget::Old ,
            _ => OpTarget::Num(num.parse().unwrap())
        };

        let (_, test) = lines.next().unwrap().split_once("divisible by ").unwrap();

        let true_target = lines.next().unwrap().split_ascii_whitespace().last().unwrap().parse().unwrap();
        let false_target = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();




        Self { items,
            items_inspected: 0,
            op: (op,op_target), 
            test: test.parse().unwrap(), 
            true_target, 
            false_target 
        }


    }

    fn inspect_next(&mut self) -> Vec<(usize, usize)>{
        //println!("{}", &self.name);
        let mut result = Vec::new();
        while let Some(item) = self.items.pop_front(){
            //println!("  Monkey inspects an item with a worry level of {}.", item);
            let (op, target) = &self.op;
            let target = match target {
                OpTarget::Num(x) => *x,
                _ => item
            };
            let value = match op{
                Op::Add => item + target,
                _=> item * target
            };
            //println!("    Worry level is {:?} by {} to {}", op,target,value);

            //value = value / 3;
            //println!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", value);
            
            let test = value % self.test == 0;
            let target_monkey = match test{
                true=> self.true_target,
                _=> self.false_target
            };
            //println!("    Current worry level is {} divisible by {}.", test, self.test);
            //println!("    Item with worry level {} is thrown to monkey {}.", value, target_monkey);
            //println!();

            result.push((value, target_monkey));
        }

        self.items_inspected += result.len();
        result
        
    }
}
fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn get_score(input: &str)->usize{
    let mut monkeys = Monkey::vec_from_text(input);
    let lcm = monkeys.iter().map(|m| m.test).reduce(lcm).unwrap();

    println!("{}", lcm);

    for _ in 0..10000{
        for index in 0.. monkeys.len(){
            let current_monkey = monkeys.get_mut(index).unwrap();
            let throws = current_monkey.inspect_next();
            for (value, target) in throws{
                monkeys.get_mut(target).unwrap().items.push_back(value % lcm);
            }
        }  
    }

    let mut score = monkeys.iter().map(|m| m.items_inspected).collect::<Vec<_>>();
    score.sort_by_key(|&c| std::cmp::Reverse(c));
    score.iter().take(2).product()
}
fn main() {

    const INPUT:&str  = include_str!("input.txt");
    let mul = get_score(INPUT);
    println!("{:?}", mul);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2() {
        const INPUT:&str  = include_str!("input.txt");
        assert_eq!(25738411485, get_score(INPUT));
    }
}
