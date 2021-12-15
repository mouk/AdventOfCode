use std::fs;

use std::collections::HashMap;
use std::path::Path;

fn read_edges<P>(path: P) -> HashMap<String, Vec<String>> 
where P: AsRef<Path>
{

    let content = fs::read_to_string(path).expect("war richtig");

    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    for line in content.split("\r\n") {
        let nodes: Vec<&str> = line.split('-').collect();
        let entry = result.entry(nodes[0].to_string()).or_insert(Vec::new());
        entry.push(nodes[1].to_string());

        let entry = result.entry(nodes[1].to_string()).or_insert(Vec::new());
        entry.push(nodes[0].to_string());
    }
    result
}
fn can_be_visited(name: &String, path: &Vec<String>, complex:bool) -> bool {
    if *name == "start" {
        return false;
    }
    if name.to_uppercase() == *name || !path.contains(name) {
        return true;
    }
    if !complex{
        return false;
    }
    for first in 0..(path.len()-1){
        if path[first] != path[first] .to_uppercase(){   
        for second in (first+1)..path.len(){
            if path[first] == path[second]{
                return false;
            }
        }
    }
    }
    return true;
}
fn dfs(map: &HashMap<String, Vec<String>>, complex:bool) -> usize
{
    let mut full_path_count:usize= 0;
    let mut stack = vec![vec!["start".to_string()]];

    while let Some(p) = stack.pop() {
        let start = p.last().unwrap();
        match map.get(start) {
            Some(candidates) => {
                for next in candidates.into_iter().filter(|x| can_be_visited(x, &p, complex)) {
                    if next == "end" {
                        full_path_count += 1;
                    } else {
                        let mut copy = p.clone();
                        copy.push(next.clone());
                        stack.push(copy);
                    }
                }
            }
            _ => {}
        }
    }
    full_path_count
}


#[test]
fn part1_simple_algorithm() {
    let map = read_edges("src/test.txt");
    assert_eq!( dfs(&map, false), 226);
    
    let actual_map = read_edges("src/input.txt");
    assert_eq!( dfs(&actual_map, false), 3450);
}


#[test]
fn part2_complex_algorithm() {
    let map = read_edges("src/test.txt");
    assert_eq!( dfs(&map, true), 3509);
}

fn main() {
    
    let map = read_edges("src/input.txt");

    let results = dfs(&map, false);

    println!("Part 1 {:?}", results);
    let results = dfs(&map,true);

    println!("Part 2 {:?}", results);
}
