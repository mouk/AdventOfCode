use std::fs;

fn f(x: &str) -> i32 {
    return x.parse::<i32>().unwrap();
}


fn main() {
    let content = fs::read_to_string("input.txt")
    .expect("war richtig");

    let lines = content
    .split("\r\n")
    .map(f)
    .collect::<Vec<i32>>();
    let count = lines.len();
    
    let mut found = 0;
    for i in 0..(count-1) {  
        if lines[i] < lines[i+1] {
            found= found+1;
        }
    }
    println!("Part1 {}", found); 

    found = 0;
    for i in 0..(count-3) {  
        if lines[i] + lines[i+1] + lines[i+2] < lines[i+1] + lines[i+2] + lines[i+3] {
            found= found+1;
        }
    }
    println!("Part2 {}", found); 
}