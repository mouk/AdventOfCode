
const INPUT_DATA: &str = include_str!("input.txt");

fn range_contained(line: ((usize,usize),(usize,usize))) -> bool{
    let (fs,fe) = line.0;
    let (ss,se) = line.1;
    (fs <= ss) && (fe >= se) || (ss <= fs) && (se >= fe)
}
fn range_overlaps(line: ((usize,usize),(usize,usize))) -> bool{
    let (fs,fe) = line.0;
    let (ss,se) = line.1;
    ((fs >= ss) && (fs <= se)) || ((fe >= ss) && (fe <= se))
    || ((se >= fs) && (se <= fe))
}

fn parse(line: &str) -> ((usize,usize),(usize,usize)){
    let l = line.split(',').map( |p| {
        let (s,e) = p.split_once('-').unwrap();
        (s.parse::<usize>().unwrap(), e.parse::<usize>().unwrap())
    })
    .collect::<Vec<_>>();
    (l[0], l[1])
}
fn main() {


    let result = INPUT_DATA
    .split('\n')
    .map(parse)
    .filter(|&l| range_contained(l))
    .count();
    println!("Part1  {}", result);


    let result = INPUT_DATA
    .split('\n')
    .map(parse)
    .filter(|&l| range_overlaps(l))
    .count();
    println!("Part2  {}", result);
}
