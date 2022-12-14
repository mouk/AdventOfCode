use std::collections::HashSet;


#[derive(Debug)]
struct Rope {
    knots:Vec<(i32, i32)>,
    visited: HashSet<(i32, i32)>
}
impl Rope {
    fn transform(mut self, direction: (i32, i32)) -> Self {


        let mut last = None;
        let new_knots:Vec<(i32, i32)>  = self.knots.iter().map(|(x,y)| {

            if let Some((xh,yh)) = last{
                let new_knot = match ((xh,yh),(*x,*y)) {
                    ((x1,y1),(x2,y2)) if x1 == x2 => (x1, Rope::calculate_tail(y1,y2)),
                    ((x1,y1),(x2,y2)) if y1== y2 => (Rope::calculate_tail(x1,x2), y1),
                    ((x1,y1),(x2,y2)) if (x1-x2).abs() < 2 && (y1-y2).abs() < 2 => (x2,y2),
                    ((x1,y1),(x2,y2))  => (x2 + (x1-x2)/(x1-x2).abs(),y2 + (y1-y2)/(y1-y2).abs())
               };
               last = Some(new_knot);
               new_knot
            }else{
                let new_head = (x + direction.0, y + direction.1);
                last = Some(new_head);
                new_head
            }

        }).collect();

        self.visited.insert(*new_knots.last().unwrap());

        Self { knots: new_knots, visited:self.visited }
    }
    fn apply_step(self, count: usize, direction: (i32, i32)) -> Self {
        let mut result = self;
        for _ in 0..count{
            result = result.transform(direction);
        }   
        result
    }
    fn calculate_tail(h:i32,t:i32) -> i32{
        assert!((h-t).abs() <=2);
        if (h-t).abs() <= 1 {
            t
        }else if h > t{
            t +1
        }else {
            t-1
        }
    }
    fn read_input(input: &str, knots:usize)-> Rope{
        let mut visited = HashSet::new();
        visited.insert((0,0));
        let knots = (0..knots).map(|_| (0,0)).collect();
        let mut start = Rope{knots:knots, visited: visited};
        for line in input.split('\n'){
            let (dir, count)= line.split_once(' ').unwrap();
            let count = count.parse::<usize>().unwrap();
            let direction = match dir{
                "R"=> (0,1),
                "L"=> (0,-1),
                "U"=> (1,0),
                "D"=> (-1,0),
                _ => panic!()
            };
            start = start.apply_step(count, direction);
            //println!("{}, {:?}", line, start);
        }
        start
    }
}
fn main() {
    const TEST:&str  = include_str!("input.txt");
    let rope = Rope::read_input(TEST, 2);
    println!("Part1 {:?}", rope.visited.len());
    let rope = Rope::read_input(TEST, 10);
    println!("Part2 {:?}", rope.visited.len());
}
