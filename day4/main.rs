use std::fs;
//use std::convert::TryFrom;

const ONE_DIM:usize = 5;
const WIDTH:usize = ONE_DIM * ONE_DIM;

struct Matrix{
    data: [i32; WIDTH]
}
impl Matrix{
    fn new(raw_data: &str) -> Self {
        let nums = raw_data
            . split_ascii_whitespace()// .split(" ")
            .map(|x| x.parse::<i32>().unwrap());

        let mut data= [0; WIDTH];
        let mut i = 0;
        for n in nums{
            data[i] = n;
            i = i + 1;
        }
        assert_eq!(i, WIDTH, "wrong umber of items");
        return Matrix{
            data: data
        };
    }

    fn get(&self, row:usize, col:usize) -> i32{
        self.data[ONE_DIM * row + col]
    }

    fn is_solution(&self,chosen: &Vec<i32>) -> bool{
        'outer: for row in 0..ONE_DIM {
            for col in 0..ONE_DIM{
                let n =  self.get(row, col) ;
                
                // println!("Item  {} {} {}  ",row, col, n);
                if !chosen.contains(&n){
                    continue 'outer;
                }
            }
            
            println!("Found at row  {} ",row);
            return true;
        }
        'outer1: for col in 0..ONE_DIM {
            for row in 0..ONE_DIM{
                let n =  self.get(row, col) ;
                
                // println!("Item  {} {} {}  ",row, col, n);
                if !chosen.contains(&n){
                    continue 'outer1;
                }
            }
            
            println!("Found at col  {} ",col);
            return true;
        }
        return false;
    }
}

fn main() {
    let content = fs::read_to_string("input.txt")
    .expect("war richtig");

    let mut org = content
    .split("\r\n");
    let chosen = org.next().unwrap().split(",").map(|x| x.parse::<i32>().unwrap());

    let mut matices: Vec<Matrix> = Vec::new();

    let mut data = String::from("");
    for x in org {
        if x != "" {
            data.push_str(" ");
            data.push_str(x)
        }else{
            if data != ""{
                println!("|{}|", data);
                let m = Matrix::new(data.as_str());
                matices.push(m);
            }
            
            data = String::from("");
        }
        
    }
    let mut chosen_numbers: Vec<i32> = Vec::new();
    let mut done: Vec<[i32; WIDTH]> = Vec::new();
    for lot in chosen{
        chosen_numbers.push(lot);
        for m in &matices{
            if done.contains(&m.data){
                continue;
            }
            if m.is_solution(&chosen_numbers){
                
                println!("|{}|",lot);
                
                println!("|{:?}|", m.data);
                println!("|{:?}|", &chosen_numbers);
                let mut prod:i32 = 0;
                for n in m.data{
                    if !chosen_numbers.contains(&n){
                     println!("|{}*{}|",prod, n);
                        prod = prod + n
                    }
                }
                
                println!("Result |{}|",prod * lot);

                //TODO remove
                done.push(m.data)
            }
        }

    }

    println!("Part1  {} ",matices.len());
    println!("Part1  {} ",matices[0].is_solution(&vec![14, 86, 50, 89, 49]));
    println!("Part1  {} ",matices[0].is_solution(&vec![14, 86, 50, 100, 89, 49]));
    println!("Part1  {} ",matices[1].is_solution(&vec![14, 86, 45, 89, 49]));

}