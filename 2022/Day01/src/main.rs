const TEST_DATA: &str = include_str!("test.txt");

fn main() {
    let mut result= TEST_DATA.split("\n\n").map(|x| x.split('\n').map(|x| x.parse::<u64>().unwrap()).sum::<u64>()).collect::<Vec<_>>();

    result.sort();

    println!("Part 1 {}", result.last().unwrap());

    println!("Part 2 {}", result[result.len()-1]+ result[result.len()-2] + result[result.len()-3])
}
