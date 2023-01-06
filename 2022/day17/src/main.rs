use std::collections::HashSet;

#[derive(Debug)]
struct Shape {
    points: Vec<Point>,
}

struct Generator {
    state: usize,
}

impl Generator {
    fn next(&mut self, height: usize) -> Shape {
        self.state = (self.state + 1 )% 5;
        let row = height + 3;
        let col = 2;
        match self.state {
            1 => Shape {
                points: vec![(row, col), (row, col + 1), (row, col + 2), (row, col + 3)],
            },
            2 => Shape {
                points: vec![
                    (row, col + 1),
                    (row + 1, col),
                    (row + 1, col + 1),
                    (row + 1, col + 2),
                    (row + 2, col + 1),
                ],
            },
            3 => Shape {
                points: vec![(row, col), (row, col + 1), (row, col + 2), (row + 1, col + 2), (row + 2, col + 2)],
            },
            4 => Shape {
                points: vec![(row, col), (row+ 1, col ), (row+ 2, col ), (row+ 3, col )],
            },
            _ => Shape {
                points: vec![(row, col), (row, col + 1), (row+ 1, col ), (row+ 1, col +1 )],
            },
        }
    }
}

#[derive(Debug)]
enum Dir {
    Right,
    Left,
    Down,
}
type Point = (usize, usize);
struct World{
    points: HashSet<Point>
}
impl World {

    fn trim(&mut self){
        if self.height() - self.min() > 100{
            let threshold = self.height()  - 50;
            //let before = self.points.len();
            self.points.retain(|v| v.0  >threshold);
            //println!("{} to {}", before,self.points.len());
        }

    }
    fn new() ->Self{
        World{
            points: HashSet::<Point>::new()
        } 
    }
    fn height(&self) -> usize {
        self.points.iter().map(|(row, _)| *row + 1).max().unwrap_or(0)
    }


    fn min(&self) -> usize {
        self.points.iter().map(|(row, _)| *row + 1).min().unwrap_or(0)
    }


    fn add(&mut self, s: Shape) {
        self.points.extend(s.points);
    }

    fn contains(&self, p: &Point)->bool {
        self.points.contains(p)
    }

    fn print(&self, additional_shape: Option<&Shape>){

        let points = match additional_shape{
            None => vec![],
            Some(s) => s.points.clone()
        };

        let height = self.height().max(points.iter().map(|s| s.0).max().unwrap_or_default());
        for r in 0..=height{
            let row = height - r;
            print!("{}\t|",row);
            for col in 0..7{
                let p = &(row, col);
                let sym = if self.contains(p){'#'}else if points.contains(p){'@'}else{'.'};
                print!("{}", sym)
            }
            print!("|");
            println!();
        }

        println!("\t---------\n");
    }
}

impl Dir {
    fn apply(&self, p: &(usize, usize)) -> Option<Point> {
        match self {
            Self::Down => {
                if p.0 == 0 {
                    None
                } else {
                    Some((p.0 - 1, p.1))
                }
            }
            Self::Left => {
                if p.1 == 0 {
                    None
                } else {
                    Some((p.0, p.1 - 1))
                }
            }
            Self::Right => {
                if p.1 == 6 {
                    None
                } else {
                    Some((p.0, p.1 + 1))
                }
            }
        }
    }
}

impl Shape {
    fn collides(&self, other: HashSet<Point>) -> bool {
        self.points.iter().any(|p| other.contains(p))
    }

    fn len(&self) -> usize {
        self.points.len()
    }
    
    fn apply(&self, dir: &Dir, w: &World) -> Option<Self> {
        let len = self.len();
        let mut points= Vec::with_capacity(len);
        for i in 0..len{
            if let Some(p) = dir.apply(&self.points[i]){
                if w.contains(&p){
                    return None
                }
                points.push(p);

            }else{
                return None;
            }

        }
        Some(Shape {
            points,
        })
    }
}

const TEST_DATA: &str = include_str!("test.txt");
const INPUT_DATA: &str = include_str!("input.txt");

fn simulate(input: &str, rounds :usize)-> usize{
    let all = 
    input
        .chars()
        .map(|c| if c == '<' { Dir::Left } else { Dir::Right })
        .collect::<Vec<_>>();

    let mut tokens = (0..)
    .map(|i| &all[i % all.len()]);

    let mut gen = Generator{state:0};
    let mut w = World::new();

    

    for i in 0..rounds{

        if i % 10 == 0 {

            w.trim();
        }

        if i % 100000 == 0 {
            println!("Loop {}", i);
        }
        let  height =  w.height();
        let mut s =gen.next(height);

        //w.print(Some(&s));
        loop {
            let op = tokens.next().unwrap();
            s = s.apply(op, &w).unwrap_or(s);
            if let Some(down) = s.apply(&Dir::Down, &w) {
                s = down
            } else {
                //w.print(Some(&s));
                break;
            }

            //w.print(Some(&s));
            
        }
        w.add(s);

    }
    w.height()

}
fn main() {
    println!("{:?}", simulate(TEST_DATA,1_000_000_000_000));
    //println!("Part 1 {:?}", result.calculate());
    //println!("Max pressure {:?}", result.calculate_with_elephant());

}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test1() {
        assert_eq!(simulate(TEST_DATA,2022), 3068)
    }


    #[test]
    fn test2() {
        assert_eq!(simulate(TEST_DATA,1_000_000_000_000), 1514285714288)
    }
    #[test]
    fn test_input() {
        assert_eq!(simulate(INPUT_DATA,2022), 3235)
    }
/*
    #[test]
    fn test_part2_input() {
        let result = Map::from_test(INPUT_DATA);
        assert_eq!(result.calculate_with_elephant(), 2679)
    }

    */
}
