use std::fs;

fn main() {
    
    let content = fs::read_to_string("input.txt")
    .expect("war richtig");

    let mut gen0= [0; 9];
    let nums= content
    .split(",")
    .map(|x| x.parse::<usize>().unwrap());
    
    for n in nums{
        gen0[n] += 1;
    }
    println!("Gen0 {:?}", gen0);

    let days = 256;
    for _ in 0..days{
        let birth = gen0[0];
        for  i in 1..gen0.len(){
            gen0[i-1]= gen0[i];
        }
        gen0[6] += birth;
        gen0[gen0.len()-1]  = birth;
    }

    let population:u64 = gen0.iter().sum();
    println!("Population after {} days is {}", days, population);   
}