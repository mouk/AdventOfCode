use std::fs;
use std::convert::TryFrom;

fn main() {
    let base: i32 = 2;


    let content = fs::read_to_string("input.txt")
    .expect("war richtig");

    let org = content
    .split("\r\n")
    .collect::<Vec<_>>();

    let positions = org.clone();

    const WIDTH:usize = 12;


    fn bin_to_dec(bin: &str)-> i32{
        let base: i32 = 2;
        let chars = bin.chars().rev();
        let mut p:u32 = 0;
        let mut result = 0;
        for c in chars {
            if c == '1'{
                result = result + base.pow(p)
            }
            p = p +1            
        }
        result
    }
    
    fn recount(positions: Vec<&str>, w:usize)->([i32;WIDTH], [i32;WIDTH]){
        let mut zeros= [0;WIDTH];
        let mut ones= [0;WIDTH];
        for pos in positions {
            assert_eq!(pos.len(), w); // fails
            let mut i = 0;
            for c in pos.chars(){
                if c == '1' {
                    ones[i]=ones[i]+1
                }else{
                    zeros[i]= zeros[i]+1
                }
                i = i + 1;
            }
        }
        (zeros,ones)
    }

    let (zeros, ones) = recount(positions, WIDTH);
    
    println!("ones is {:?}",ones);
    println!("zeros is {:?}",zeros);

    let mut gamma  = 0;
    let mut epsilon   = 0;
    
    for  i in 0..WIDTH {
        if ones[i] > zeros[i]{
            gamma  += base.pow(u32::try_from(WIDTH -1 -i).unwrap())
        }else{
            epsilon  += base.pow(u32::try_from(WIDTH -1 -i).unwrap())
            
        }
    }
    println!("Part1 {} * {} = {} ", gamma, epsilon, epsilon *  gamma);
    let mut oxygen = org.clone();
    let mut co2 = org.clone();

    
    println!("oxygen is {:?}",oxygen[0]);
    println!("co2 is {:?}",co2[0]);

    for  i in 0..WIDTH {
        let (zeros, ones) = recount(oxygen.clone(), WIDTH);
        let expected =  if  ones[i] >= zeros[i] {'1'}else{'0'};
        oxygen = oxygen.iter().cloned().filter(|p| p.chars().nth(i).unwrap() == expected).collect();
        println!("left {}",oxygen.len());
        
        println!("After filtering for {} oxygen is {:?}",expected, oxygen);
        if oxygen.len() == 1{
            break
        }
    }
    
    for  i in 0..WIDTH {
        let (zeros, ones) = recount(co2.clone(), WIDTH);
        let expected =  if  ones[i] >= zeros[i] {'0'}else{'1'};
        co2 = co2.iter().cloned().filter(|p| p.chars().nth(i).unwrap() == expected).collect();
        println!("left {}",co2.len());
        if co2.len() == 1{
            break
        }
    }

    
    println!("oxygen is {:?} {}",oxygen, bin_to_dec(oxygen[0]));
    println!("co2 is {:?} {}",co2, bin_to_dec(co2[0]));
    println!("Part2  {} ",  bin_to_dec(oxygen[0]) *bin_to_dec(co2[0]) );

}