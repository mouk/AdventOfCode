use std::{collections::HashSet, fs, path::Path};

#[derive(Debug)]
enum FoldInstruction {
    X(usize),
    Y(usize),
}
fn fold(p: usize, at: usize) -> usize {
    if p < at {
        return p;
    }
    return 2 * at - p;
}

fn apply_instruction(
    points: HashSet<(usize, usize)>,
    ins: &FoldInstruction,
) -> HashSet<(usize, usize)> {
    let transformed = points.iter().map(|(x, y)| match ins {
        FoldInstruction::X(at) => (fold(*x, *at), *y),
        FoldInstruction::Y(at) => (*x, fold(*y, *at)),
    });

    HashSet::from_iter(transformed)
}

fn apply_instructions(
    points: HashSet<(usize, usize)>,
    instructions: Vec<FoldInstruction>,
) -> HashSet<(usize, usize)> {
    let mut myp = points;
    for ins in instructions {
        myp = apply_instruction(myp, &ins);
    }
    myp
}

fn print(points: &HashSet<(usize, usize)>) {
    let max_x = *points.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *points.iter().map(|(_, y)| y).max().unwrap();

    // 8 letters
    let seperator_after = (max_x +1) / 7; 
    
    println!("");
    for y in 0..=max_y {
        for x in 0..=max_x {
            let c = match points.get(&(x, y)) {
                Some(_) => "#",
                _ => " ",
            };
            print!("{}", c);
            if (x+1)% seperator_after == 0 {
                print!("\t");
            }
        }
        println!("")
    }
    println!("");
}

fn read_edges<P>(path: P) -> (HashSet<(usize, usize)>, Vec<FoldInstruction>)
where
    P: AsRef<Path>,
{
    let content = fs::read_to_string(path).expect("war richtig");
    let (points, instructions) = content.split_once("\r\n\r\n").unwrap();
    let points = points.split("\r\n").map(|line| {
        let mut co = line.split(",").map(|x| x.parse::<usize>().unwrap());

        return (co.next().unwrap(), co.next().unwrap());
    });

    let ins = instructions
        .split("\r\n")
        .map(|line| {
            let (axe, at) = line.split_once("=").unwrap();
            let at = at.parse::<usize>().unwrap();
            match axe {
                "fold along y" => FoldInstruction::Y(at),
                _ => FoldInstruction::X(at),
            }
        })
        .collect();

    (HashSet::from_iter(points), ins)
}

fn main() {
    let (points, instructions) = read_edges("src/input.txt");
    let result = apply_instruction(points, &instructions[0]).len();
    println!("Part1 {:?}", result);

    let (points, instructions) = read_edges("src/input.txt");
    let result = apply_instructions(points, instructions);
    print(&result);
}

#[test]
fn part1() {
    let (points, instructions) = read_edges("src/test.txt");
    let result = apply_instruction(points, &instructions[0]).len();
    assert_eq!(result, 17);

    let (points, instructions) = read_edges("src/input.txt");
    let result = apply_instruction(points, &instructions[0]).len();
    assert_eq!(result, 729);
}

#[test]
fn part2() {
    let (points, instructions) = read_edges("src/test.txt");
    assert_eq!(apply_instructions(points, instructions).len(), 16);

    let (points, instructions) = read_edges("src/input.txt");
    assert_eq!(apply_instructions(points, instructions).len(), 100);
}
