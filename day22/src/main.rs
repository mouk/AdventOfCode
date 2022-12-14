use min_max::*;
use itertools::Itertools;
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Cube {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    z_min: i64,
    z_max: i64,
    sign: i64
}
impl Cube {
    
   

    fn len(&self) -> i64 {
        (1 + self.x_max - self.x_min)  as i64
            * (1 + self.y_max - self.y_min)  as i64
            * (1 + self.z_max - self.z_min)  as i64
            * self.sign
    }
    fn new_from_tuples(add: bool, xp: (i64, i64), yp: (i64, i64), zp: (i64, i64)) -> Self {
        assert!(xp.0 <= xp.1, "X invalid");
        assert!(yp.0 <= yp.1, "Y invalid");
        assert!(zp.0 <= zp.1, "Z invalid");
        Self {
            x_min: xp.0,
            x_max: xp.1,
            y_min: yp.0,
            y_max: yp.1,
            z_min: zp.0,
            z_max: zp.1,
            sign: if add {1}else{-1}
        }
    }
    fn new_from_line(line: &str) -> Self {
        //println!("Processing line {}", line);
        let (ins, line) = line.split_once(" ").unwrap();
        let add = ins == "on";
        assert!(add || ins == "off");
        let (xp,yp,zp) = line.split(",").map(to_tuple).collect_tuple().unwrap();
        Cube::new_from_tuples(add,xp, yp, zp)
    }

    fn intersection(&self, other: &Self) -> Option<Self>{
        let x_min = max!(self.x_min, other.x_min);
        let x_max = min!(self.x_max,other.x_max);
        let y_min = max!(self.y_min, other.y_min);
        let y_max = min!(self.y_max,other.y_max);
        let z_min = max!(self.z_min, other.z_min);
        let z_max = min!(self.z_max,other.z_max);

        if x_min > x_max || y_min > y_max || z_min > z_max{
            return None;
        }

        let sign = if self.sign == other.sign{ - self.sign}else if self.sign == 1{1} else{-1};

        Some(Self {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
            sign
        })
    }
}

fn to_tuple(r: &str) ->  (i64, i64) {
    let r = &r[2..];
    let (start, end) = r
        .split("..")
        .map(|x| x.parse::<i64>().unwrap())
        .collect_tuple()
        .unwrap();

    assert!(start <= end, "start cannot be greater than end");
    (start, end)
}
fn new_from_file(content: &str) -> i64 {
    let mut result: Vec<Cube> = Vec::new();
    content
    .split("\n")
    .map(Cube::new_from_line)
    .for_each(|n_cube| {  
        let sign = (&n_cube.sign).clone();  
        for cube in result.clone(){
            if let Some(intersection) = n_cube.intersection(&cube){
                result.push(intersection);
            }
            
            
        }
        if sign == 1 {
            result.push(n_cube);      
        }
    });

    result.iter().map(Cube::len).sum()
}

fn main() {
    const INPUT_DATA: &str = include_str!("input2.txt");
    let result = new_from_file(INPUT_DATA);

    println!("Part2 {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST2_DATA: &str = include_str!("test2.txt");
    const TEST_DATA: &str = include_str!("test.txt");
    #[test]
    fn test_prt1() {
        let result = new_from_file(TEST_DATA);
        println!("test1 {:?}", result);
        assert_eq!(result, 39)
    }
    #[test]
    fn basic() {
        assert_eq!(Cube::new_from_tuples(true, (10,10), (10,10), (10,10)).len(), 1);
        assert_eq!(Cube::new_from_tuples(true, (10,11), (9,10), (10,10)).len(), 4);
        assert_eq!(Cube::new_from_tuples(false, (10,11), (9,10), (10,11)).len(), -8);

        let c1 = Cube::new_from_tuples(true, (10,10), (10,10), (10,10));
        let c2 = Cube::new_from_tuples(false, (10,10), (10,10), (10,10));
        println!("{:?}", &c1.intersection(&c2))
    }
    #[test]
    fn test2_prt1() {
        let result = new_from_file(TEST2_DATA);

        assert_eq!(result, 590784);
    }
    #[test]
    fn test3_part2() {
        const TEST3_DATA: &str = include_str!("test3.txt");
        let result = new_from_file(TEST3_DATA);

        assert_eq!(result, 2758514936282235);
    }
}
