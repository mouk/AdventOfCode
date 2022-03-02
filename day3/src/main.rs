use std::fs;
use std::convert::TryFrom;
use bitvec::prelude::*;

fn main() {
    let base: i32 = 2;


    let content = fs::read_to_string("input.txt")
    .expect("No input");

    let org = content
    .split("\n")
    .collect::<Vec<_>>();

    let positions = org.clone();

    const WIDTH:usize = 12;


    fn bin_to_dec(bin: &str)-> i32{
        i32::from_str_radix(bin, 2).unwrap()
    }
    
    fn count(positions: Vec<&str>)->([i32;WIDTH], [i32;WIDTH]){
        let mut zeros= [0;WIDTH];
        let mut ones= [0;WIDTH];
        for pos in positions {
            assert_eq!(pos.len(), WIDTH); // fails
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

    let (zeros, ones) = count(positions);

    let mut gamma  = 0;
    let mut epsilon   = 0;
    
    for  i in 0..WIDTH {
        if ones[i] > zeros[i]{                            
            gamma  += base.pow(u32::try_from(WIDTH -1 -i).unwrap())
        }else{
            epsilon  += base.pow(u32::try_from(WIDTH -1 -i).unwrap())
            
        }

    }
    let part1 =  epsilon *  gamma;
    assert_eq!(part1, 852500);
    println!("Part1 {} * {} = {} ", gamma, epsilon,part1);
    let mut oxygen = org.clone();
    let mut co2 = org.clone();


    for  i in 0..WIDTH {
        let (zeros, ones) = count(oxygen.clone());
        let expected =  if  ones[i] >= zeros[i] {'1'}else{'0'};
        oxygen = oxygen.iter().cloned().filter(|p| p.chars().nth(i).unwrap() == expected).collect();

        if oxygen.len() == 1{
            break
        }
    }
    
    for  i in 0..WIDTH {
        let (zeros, ones) = count(co2.clone());
        let expected =  if  ones[i] >= zeros[i] {'0'}else{'1'};
        co2 = co2.iter().cloned().filter(|p| p.chars().nth(i).unwrap() == expected).collect();
        if co2.len() == 1{

            break
        }
    }

    let part2 = bin_to_dec(oxygen[0]) *bin_to_dec(co2[0]) ;
    assert_eq!(bin_to_dec(oxygen[0]), 2235);
    assert_eq!(bin_to_dec(co2[0]), 451);
    assert_eq!(part2, 1007985);
    
    println!("oxygen is {:?} {}",oxygen, bin_to_dec(oxygen[0]));
    println!("co2 is {:?} {}",co2, bin_to_dec(co2[0]));
    println!("Part2  {} ",  part2);

    

}
 