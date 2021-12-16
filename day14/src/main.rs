use std::{collections::HashMap, path::Path, fs};
use std::string::*;
use counter::Counter;

#[derive(Debug)]
struct Polymerization{
    template:String,
    insertions:HashMap<String,String>,
    age: usize
}

impl Polymerization{
    
    fn get_diff(&mut self)->usize{
         let map = self
        .template.chars()
        .collect::<Counter<_>>();

        let frequencies = map.values().map(|x| *x).collect::<Vec<usize>>();

        let max = frequencies.iter().max().unwrap();
        let min = frequencies.iter().min().unwrap();
        *max - *min
    }
    fn add_year(&mut self){
        let mut result = String::new();
        let parent = &self.template;
        for i in 1..parent.len(){
            result.push_str(parent.get((i-1)..i).unwrap());
            
            if let Some(key) = parent.get((i-1)..=i){
                if let Some(insertion) = self.insertions.get(key){
                    result.push_str(insertion);
                }
            }
        }
        
        result.push_str(parent.get((parent.len()-1)..(parent.len())).unwrap());
        self.template =  result.to_string();
        self.age += 1;
    }
    
    fn add_years(&mut self, years: usize){
       for _ in 0..years{
           self.add_year();
       }
    }
}

fn get_input<P>(path: P) -> Polymerization
where P: AsRef<Path>
{

    let content = fs::read_to_string(path).expect("invalid");
    
    let (template, rules) = content.split_once("\n\n").unwrap();

    let pairs = rules
    .split("\n")
    .map(|line| {
        let (left, right) = line.split_once(" -> ").unwrap();
        (left.to_string(),right.to_string())
});
    let insertions:HashMap<String,String> = HashMap::from_iter(pairs);

    return Polymerization{
        template:template.to_string(),
        insertions,
        age:0
    }
}

fn main() {
    let mut poly = get_input("src/input.txt");
    poly.add_years(10);
    println!("Diff {:?}", poly.get_diff());

}

#[test]
fn test_data() {
    let mut poly = get_input("src/test.txt");
    poly.add_years(10);
    assert_eq!( poly.get_diff(), 1588);


    poly.add_years(30);
    assert_eq!( poly.get_diff(), 2188189693529);

}


#[test]
fn test_input_data() {
    let mut poly = get_input("src/input.txt");
    poly.add_years(10);
    assert_eq!( poly.get_diff(), 2975);
}

