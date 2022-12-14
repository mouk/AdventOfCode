#[derive(Debug,Clone,Copy)]
enum Op{
    Noop,
    Add(i32)
}

struct Memory{
    x: i32,
    sampling: i32,
    cycle: usize,
    screen: String
}


impl Memory{
    fn next_cycle(&mut self){
        self.draw();
        self.cycle += 1;
        if (self.cycle + 20) % 40 == 0 {
            //println!("=====> Adding {}*{}", self.cycle, self.x);
            self.sampling += self.cycle as i32 * self.x;
        }        
    }

    fn draw(&mut self){

        if self.cycle % 40 == 0 {
            self.screen.push_str("\n");
        }

        let c = (self.cycle % 40) as i32;
        let output = if  self.x -1 <= c  &&  c  <= self.x  + 1   {"#"}else{" "};
        self.screen.push_str(output);
        /*
        println!("Register       : {}", &self.x);
        println!("During cycle {}: CRT draws pixel in position {}", self.cycle + 1, self.cycle);
        println!("Current CRT row: {}", &self.screen[1..]);
        print!( "Sprite position: ");
        for i in 0..40{

            let o = if  self.x -1 <= i  &&  i  <= self.x  + 1   {"#"}else{"."};
            print!("{}", o);
        }
        println!();
        println!();
        */


        
    }
    fn new()-> Self{
        Memory{
            x: 1,
            cycle: 0,
            sampling: 0,
            screen: "".to_owned()
        }
    }
    fn apply(&mut self, op: Op){
        self.next_cycle();
        if  let Op::Add(x) = op {
            self.next_cycle();
            self.x += x;
        }
    }

    fn apply_program(&mut self, ops: Vec<Op>) -> i32{
        for op in ops {
            self.apply(op);
        }
        self.sampling
    }
}

impl Op{
    fn from_text(input: &str) -> Vec<Op>{
        input
        .split('\n')
        .map(|l| {
             if l == "noop" {
                return Op::Noop
             }
             let (_,val) = l.split_once(' ').unwrap();
             return Op::Add(val.parse().unwrap());
        })
        .collect()
    }
}

fn main() {

    const INPUT:&str  = include_str!("input.txt");
    let mut x = Memory::new();
    let ops = Op::from_text(INPUT);
    let p1 = x.apply_program(ops);

    println!("Part 1{}", p1);

    println!("Part 2{}", x.screen);
    

}
