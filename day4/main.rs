use std::fs;
//use std::convert::TryFrom;

const ONE_DIM:usize = 5;
const WIDTH:usize = ONE_DIM * ONE_DIM;

struct Matrix{
    data: [i32; WIDTH],
    is_solved: bool
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
            data: data,
            is_solved: false
        };
    }

    fn mark_as_solved(&mut self){
        self.is_solved = true;
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
    for lot in chosen{
        chosen_numbers.push(lot);
        for m in matices.iter_mut().filter(|m| !m.is_solved){
            if m.is_solution(&chosen_numbers){
                m.mark_as_solved();
                println!("|{}|",lot);
                
                println!("|{:?}|", m.data);
                println!("|{:?}|", &chosen_numbers);
                let sum:i32 = m.data.iter().filter(|n| !chosen_numbers.contains(n)).sum();
                
                println!("Result |{}|",sum * lot);
            }
        }

    }
}