use std::fs;

use std::collections::HashMap;

fn read_input()-> HashMap<String,Vec<String>>{
    let content = fs::read_to_string("./src/input.txt")
    .expect("war richtig");

    let mut result:HashMap<String,Vec<String>> = HashMap::new();
    for line in content.split("\r\n"){
        let nodes:Vec<&str> = line.split('-').collect();
        let entry = result.entry(nodes[0].to_string()).or_insert(Vec::new());
        entry.push(nodes[1].to_string());

        let entry = result.entry(nodes[1].to_string()).or_insert(Vec::new());
        entry.push(nodes[0].to_string());
    }
    result
}

fn can_be_visited_simple(name: &String, path:&Vec<String>) -> bool{
    if *name == "start"{
        return false;
    }
    if name.to_uppercase() == *name || !path.contains(name){
        return true;
    }
    return false;
}
fn can_be_visited(name: &String, path:&Vec<String>) -> bool{
    if *name == "start"{
        return false;
    }
    if name.to_uppercase() == *name || !path.contains(name){
        return true;
    }
    let mut copy = path.clone();
    copy.dedup();
    copy.len() == path.len()

}
fn dfs<P>(map: &HashMap<String,Vec<String>>, path:Vec<String>, results:&mut Vec<Vec<String>>, pred: &P)
where
  P: Fn(&String, &Vec<String>) -> bool{
    let start = path.last().unwrap().clone();
    let possible_targets = map.get(&start);
    match possible_targets {
        Some(v) => {
            for next in v.into_iter().filter(|x| pred(x, &path)) {
                let mut copy = path.clone();
                copy.push(next.clone());
                if next == "end"{
                    results.push(copy);
                }
                else {
                    dfs(map,copy,results, pred);
                }
            }  
        }
        None => {}       
    }   
}

fn print_result(results:&Vec<Vec<String>>){
    for r in results{
        println!("->{:?}", r);
    }
}

fn main() {
    let map = read_input();
    let start = vec!["start".to_string()];
    let mut results:Vec<Vec<String>> = Vec::new();
    dfs(&map,start,&mut results, &can_be_visited_simple);

    print_result(&results);
    println!("Part 1 {:?}", results.len());


    /* Stackoverflow */
    
    let start = vec!["start".to_string()];
    let mut results:Vec<Vec<String>> = Vec::new();
    dfs(&map,start,&mut results, &can_be_visited);

    print_result(&results);
    println!("Part 2 {:?}", results.len());
}