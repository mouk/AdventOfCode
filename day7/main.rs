use std::fs;
use std::collections::HashMap;
fn abs_diff(a:u16, b:u16) -> u16{
    if a > b{
        return a-b;
    }
    return b-a;
}

fn main() {
    let content = fs::read_to_string("input.txt")
    .expect("war richtig");


    let nums= content
    .split(",")
    .map(|x| x.parse::<u16>().unwrap());
    
    let mut dict=HashMap::<u16,u32>::new();
    for n in nums{
        let result = dict.entry(n).or_insert(0); 
        *result += 1;
    }
    let min:u16 = *dict.keys().min().unwrap();
    let max:u16 = *dict.keys().max().unwrap();
    let count:u32 = dict.values().map(|h| *h).sum();
    println!("Crabs {:?} , Different dinstances {:?}, Min: {:?}, Max:{:?}", count, dict.len(), min, max);
    let results:Option<u32> = (min..max)
    .map(|h| {
        dict.iter().map(|(key, count)| u32::from(abs_diff(*key,  h)) * count).sum() 
    })
    .min();
 
    println!("Part 1 Result  {}", results.unwrap());

    
    let second_result:Option<u32> = (min..max)
    .map(|h| {
        dict.iter().map(|(key, count)| {
            let fuel = (1..=u32::from(abs_diff(*key,  h))).sum::<u32>();
            return fuel * count;
        }).sum() 
    })
    .min();
 
    println!("Part 2 Result  {}", second_result.unwrap()); 

}