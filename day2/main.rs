use std::fs;

fn f(x: &str) -> (&str, i32) {
    let mut i=  x.split(" ");
    return (i.next().unwrap(), i.next().unwrap().parse::<i32>().unwrap())
}


fn main() {
    let content = fs::read_to_string("input.txt")
    .expect("war richtig");

    let commands = content
    .split("\r\n")
    .map(f);

    let mut horizontal = 0;
    let mut depth = 0;
    for command in commands {
        let n = command.1;
        match command.0 {
            "forward" =>  horizontal += n,
            "down" => depth += n,
            _ => depth -= n,  
            }
        
    }
    println!("Part1 {}*{} = {}", horizontal, depth, depth * horizontal); 

    let commands2 = content
    .split("\r\n")
    .map(f);

    horizontal = 0;
    depth = 0;
    let mut aim = 0;
    for command in commands2 {
        let n = command.1;
        match command.0 {
            "forward" =>  {
                horizontal += n;
                depth += aim * n;
            },
            "down" =>aim += n,
            _ => aim -= n,  
            }
        
    }
    
    
    println!("Part2 {}*{} = {}", horizontal, depth, depth * horizontal); 
}
