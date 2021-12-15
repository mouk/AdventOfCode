use std::{collections::HashSet, fs, path::Path};

#[derive(Debug)]
enum Axe {
    X,
    Y,
}

#[derive(Debug)]
struct FoldInstruction {
    at: usize,
    axe: Axe,
}

fn applay_instruction(
    points: HashSet<(usize, usize)>,
    ins: &FoldInstruction,
) -> HashSet<(usize, usize)> {
    let transformed: Vec<(usize, usize)> = match ins.axe {
        Axe::X => points
            .iter()
            .filter(|(x, _)| *x != ins.at)
            .map(|(x, y)| {
                let new_x = match *x < ins.at {
                    true => *x,
                    _ => (2 * ins.at) - *x,
                };
                (new_x, *y)
            })
            .collect(),
        Axe::Y => points
            .iter()
            .filter(|(_, y)| *y != ins.at)
            .map(|(x, y)| {
                let new_y = match *y < ins.at {
                    true => *y,
                    _ => (2 * ins.at) -*y,
                };
                (*x, new_y)
            })
            .collect(),
    };
    HashSet::from_iter(transformed)
}

fn applay_instructions(
    points: HashSet<(usize, usize)>,
    instructions: Vec<FoldInstruction>,
) -> usize {
    let mut myp = points;
    println!("{:?}", &myp);
    for ins in instructions {
        myp = applay_instruction(myp, &ins);
    }
    
    println!("End result");
    println!("{:?}", &myp);
    myp.len()
}

fn read_edges<P>(path: P) -> (HashSet<(usize, usize)>, Vec<FoldInstruction>)
where
    P: AsRef<Path>,
{
    let content = fs::read_to_string(path).expect("war richtig");
    let mut parts = content.split("\r\n\r\n");
    let points = parts.next().unwrap().split("\r\n").map(|line| {
        let mut co = line.split(",").map(|x| x.parse::<usize>().unwrap());

        return (co.next().unwrap(), co.next().unwrap());
    });

    let ins = parts
        .next()
        .unwrap()
        .split("\r\n")
        .map(|line| {
            let mut components = line.split("=");
            let axe = match components.next().unwrap() {
                "fold along y" => Axe::Y,
                _ => Axe::X,
            };
            let at = components.next().unwrap().parse::<usize>().unwrap();
            FoldInstruction { at, axe }
        })
        .collect();

    (HashSet::from_iter(points), ins)
}

fn main() {
    let (points, instructions) = read_edges("src/input.txt");
    let result = applay_instruction(points, &instructions[0]).len();
    println!("Part1 {:?}", result);

    
    let (points, instructions) = read_edges("src/input.txt");
    let result = applay_instructions(points, instructions);
    println!("Part2 {:?}", result);
}

#[test]
fn part1() {
    let (points, instructions) = read_edges("src/test.txt");
    let result = applay_instruction(points, &instructions[0]).len();
    assert_eq!(result, 17);

    let (points, instructions) = read_edges("src/input.txt");
    let result = applay_instruction(points, &instructions[0]).len();
    assert_eq!(result, 729);

}

#[test]
fn part2() {
    let (points, instructions) = read_edges("src/test.txt");
    assert_eq!(applay_instructions(points, instructions), 16);
}
