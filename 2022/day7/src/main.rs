
const INPUT_DATA: &str = include_str!("input.txt");
fn proceed<'a>(found: &mut Vec<(String,usize)>,lines: &mut impl Iterator<Item=&'a str>, prefix: String)-> usize{
    let mut size :usize = 0;
    while let Some(l) = lines.next(){
        let mut parts = l.split_ascii_whitespace();
        let first = parts.next().unwrap();
        if first== "$" {
            let command = parts.next().unwrap();
            if command == "cd"{
                let target = parts.next().unwrap();
                if target == ".." {
                    break;
                }
                size += proceed(found, lines,  format!("{}{}/",prefix, target));
            }

        }else if first== "dir" {    
        }else{
            size += first.parse::<usize>().unwrap();
        }
    }
    found.push((prefix,size));
    size
}

fn process_input(input: &str)-> Vec<(String,usize)>{
    let mut result = Vec::<(String,usize)>::new();
    let mut lines = input.split('\n').into_iter();
    lines.next();
    proceed(&mut result, &mut lines, "/".to_owned());


    result
}
fn solve1(input: &str)-> usize{
    let result = process_input(input);
    result.iter().map(|(_,size)| *size).filter(|size| *size < 100000).sum()
}

fn solve2(input: &str)-> usize{
    const FILESYSTEM:usize = 70000000;
    let result = process_input(input);
    let (_,occupied )=  result.last().unwrap();
    let available = FILESYSTEM - occupied;
    let required = 30000000 - available;

    result.iter().map(|(_,size)| *size).filter(|size| *size >= required).min().unwrap()
}

fn main() {
    let result = solve1(INPUT_DATA);
    println!("Part1: {}", result);
    let result = solve2(INPUT_DATA);
    println!("Part2: {}", result);
}


#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA: &str = include_str!("test.txt");
    #[test]
    fn test1() {
        let result = solve1(TEST_DATA);
        assert_eq!(result, 95437)
    }
}

