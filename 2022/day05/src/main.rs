
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Ship {
    stacks:Vec<Vec<char>>
}
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Move {
    from: usize,
    to: usize,
    count: usize
}

impl Move{
    fn from_line(line: &str) -> Self{
        let mut tokens = line.split_ascii_whitespace();
        tokens.next();
        let count = tokens.next().unwrap().parse::<usize>().unwrap();
        tokens.next();
        let from = tokens.next().unwrap().parse::<usize>().unwrap();
        tokens.next();
        let to = tokens.next().unwrap().parse::<usize>().unwrap();
        Self{
            from:from,
            to:to,
            count:count
        }
        

    }
}

impl Ship{

    fn apply(&mut self, m: Move ){

        let mut tmp = Vec::new();

        let from = &mut self.stacks.get_mut(m.from - 1).unwrap();
        for _ in 0..m.count{
            tmp.push(from.pop().unwrap())
        }

        let target = &mut self.stacks.get_mut(m.to - 1).unwrap();
        target.append(&mut tmp);
    }
    

    fn apply2(&mut self, m: Move ){

        let mut tmp = Vec::new();

        let from = &mut self.stacks.get_mut(m.from - 1).unwrap();
        for _ in 0..m.count{
            tmp.push(from.pop().unwrap())
        }

        let target = &mut self.stacks.get_mut(m.to - 1).unwrap();
        for _ in 0..m.count{
            target.push(tmp.pop().unwrap())
        }
    }
    
    fn from_input(input: &str)-> Self{
        let lines = input.
        split('\n')
        .map(|l| l.chars().collect::<Vec<_>>())
        .rev()
        .skip(1)
        .collect::<Vec<_>>();

        let width = lines[0].len();
        let mut stacks = Vec::<Vec<char>>::new();
        for index in (1..width).step_by(4) {
            let mut one = Vec::<char>::new();
            for l in &lines {
                if l[index] != ' '{

                one.push(l[index].clone());
                }
            }
            stacks.push(one);
        }
        Self{
            stacks:stacks
        }

    }

    fn top_stack(&self)-> String{
        let mut ret: String = "".to_owned();
        for stack in &self.stacks{
            ret.push(*stack.last().unwrap());
        }
        ret
    }

}

fn parse(input: &str)->Ship{
    let (game,steps) = input.split_once("\n\n").unwrap();
    let mut ship = Ship::from_input(game);
    for step in steps.split('\n'){
        let m  = Move::from_line(step);
        ship.apply(m);
    }
    ship
}


fn parse2(input: &str)->Ship{
    let (game,steps) = input.split_once("\n\n").unwrap();
    let mut ship = Ship::from_input(game);
    for step in steps.split('\n'){
        let m  = Move::from_line(step);
        ship.apply2(m);
    }
    ship
}

const INPUT_DATA: &str = include_str!("input.txt");

const TEST_DATA: &str = include_str!("test.txt");
fn main() {

    let parsed = parse(INPUT_DATA);
    println!("Part1 {:?}", parsed.top_stack());


    let parsed = parse2(INPUT_DATA);
    println!("Part2 {:?}", parsed.top_stack());
}
