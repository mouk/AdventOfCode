use std::{ collections::HashMap};

struct Jungle{
    trees: HashMap<(usize,usize),usize>,
    width: usize
}
impl Jungle{
    fn from_input(input:&str)-> Self{
        let mut trees = HashMap::<(usize,usize),usize>::new();
        let mut width = 0;

        for (row, l) in input.split('\n').enumerate(){
            width = l.len();
            for (col,c) in l.char_indices(){
                trees.insert((row,col), c.to_digit(10).unwrap() as usize);
            } 
        }
        Jungle{
            trees:trees,
            width: width
        }
    }

    fn can_be_seen(&self, point: &(usize,usize)) -> bool{
        let (r,c) = *point;
        let width = self.width;
        let height = self.trees.get(point).unwrap();

        let groups = [
            (0..r).map(|rr| (rr,c)).collect::<Vec<(usize,usize)>>(),
            ((r+1)..width).map(|rrr| (rrr,c)).collect(),
            (0..c).map(|cc| (r,cc)).collect(),
            ((c+1)..width).map(|cc| (r,cc)).collect()
        ];
        for g in groups {
            if g.iter().all(|k| self.trees.get(k).unwrap() <height) {
                return true
            }

        }
        false

    }
    fn update_visibility(&self)-> usize{
        let width = self.width;
        let mut count = 0;

        for row in 0..width{
            for col in 0 ..width{
                let point = (row,col);
                if self.can_be_seen(&point){
                    //self.trees.insert(point, (*h,true));
                    count += 1;
                }
            }
        }

        count        
    }

    fn get_scenic_score(&self, point: &(usize,usize))-> usize{
        let (r,c) = *point;
        let width = self.width;
        let height = self.trees.get(point).unwrap();

        let mut score = 1;
        let groups = [
            (0..r).map(|rr| ((r - rr -1),c)).collect::<Vec<(usize,usize)>>(),
            (0..c).map(|cc| (r,(c - cc -1))).collect(),
            ((r+1)..width).map(|rrr| (rrr,c)).collect(),
            ((c+1)..width).map(|cc| (r,cc)).collect()
        ];
        println!("=========");

        println!("{:?} {}", point, height);
       // println!("{:?}", groups);

        for g in groups{
            let mut f = 0;
            for p in g{
                f += 1;
                if self.trees.get(&p).unwrap()  >= height{
                    break;
                }
            }
            print!("{:?} X ", f);
            score *= f;
        }
        score
    }
    fn get_scenic_score_all(&self)-> usize{
        let width = self.width;
        let mut max = 0;

        for row in 0..width{
            for col in 0 ..width{
                let point = (row,col);
                let score = self.get_scenic_score(&point);
                if score > max{
                    max = score
                }
            }
        }

        max        
    }
}
fn main() {
    const INPUT:&str  = include_str!("input.txt");
    let j = Jungle::from_input(INPUT);
    //println!("Part 1 {}", j.update_visibility());

    println!("Part 2 {}", j.get_scenic_score_all())

}
