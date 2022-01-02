use std::collections::HashSet;

use itertools::*;
#[derive(Clone, Eq, PartialEq, Debug)]
struct Cube {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

fn split_range(x_min: i32, x_max: i32, x2_min: i32, x2_max: i32) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    if x_min < x2_min && x2_min <= x_max {
        println!("Split case 1");
        result.push((x_min, x2_min - 1));
        if x2_max < x_max {
            result.push((x2_min, x2_max));
            result.push((x2_max + 1, x_max));
        } else {
            result.push((x2_min, x_max));
        }
    } else if x_min <= x2_max && x2_max < x_max {
        println!("Split case 2");
        result.push((x_min, x2_max));
        result.push((x2_max + 1, x_max));
    } else {
        println!("Split case 3");
        result.push((x_min, x_max));
    }

    println!("Split {:?} {:?} BY {:?} {:?}", x_min, x_max, x2_min, x2_max);
    println!("Result {:?}", result);
    result
}

impl Cube {
    fn overlaps(&self, other: &Self) -> bool {
        self.corners().iter().any(|x| other.contains(x))
            || other.corners().iter().any(|x| self.contains(x))
    }

    fn len(&self) -> usize {
        (1 + self.x_max - self.x_min) as usize
            * (1 + self.y_max - self.y_min) as usize
            * (1 + self.z_max - self.z_min) as usize
    }

    fn corners(&self) -> [(i32, i32, i32); 8] {
        let res = [
            (self.x_min, self.y_min, self.z_min),
            (self.x_max, self.y_min, self.z_min),
            (self.x_max, self.y_max, self.z_min),
            (self.x_max, self.y_max, self.z_max),
            (self.x_max, self.y_min, self.z_max),
            (self.x_min, self.y_max, self.z_min),
            (self.x_min, self.y_max, self.z_max),
            (self.x_min, self.y_min, self.z_max),
        ];
        res
    }

    fn new_from_tuples(xp: (i32, i32), yp: (i32, i32), zp: (i32, i32)) -> Self {
        Self {
            x_min: xp.0,
            x_max: xp.1,
            y_min: yp.0,
            y_max: yp.1,
            z_min: zp.0,
            z_max: zp.1,
        }
    }

    fn assert_no_overlapping(cubes: &Vec<Self>) {
        let overlapping = cubes
            .iter()
            .tuple_combinations()
            .filter(|(c1, c2)| !c1.overlaps(c2))
            .collect_vec();

        if overlapping.len() != 0 {
            for c in overlapping {
                println!("Overlapping {:?} ", c);
            }
        }

        assert_eq!(overlapping.len(), 0, "No overlapping");
    }
    fn subtract(&self, other: &Self) -> Vec<Self> {
        if !self.overlaps(other) {
            return vec![self.clone()];
        }
        let mut res = Vec::new();

        let mut discarded = Vec::new();
        let x_range = split_range(self.x_min, self.x_max, other.x_min, other.x_max);
        let y_range = split_range(self.y_min, self.y_max, other.y_min, other.y_max);
        let z_range = split_range(self.z_min, self.z_max, other.z_min, other.z_max);
        //println!("X {:?} ", x_range);
        //println!("Y {:?} ", y_range);
        //println!("Z {:?} ", z_range);
        for xp in x_range {
            for yp in y_range.clone() {
                for zp in z_range.clone() {
                    let cube = Cube::new_from_tuples(xp, yp, zp);
                    if !other.overlaps(&cube) {
                        res.push(cube);
                    } else {
                        discarded.push(cube);
                    }
                }
            }
        }

        assert!(discarded.len() <= 1, "Max one cube can be discarded");
        let new_size = res.iter().map(Cube::len).sum::<usize>();
        let old_size = self.len();
        let other_size = other.len();
        let c: usize = new_size + discarded[0].len();

        if c != self.len() {
            println!("Original {:?}", self);
            println!("Split to {:?} items:  {:?}", res.len(), res);
        }
        Cube::assert_no_overlapping(&res);

        assert_eq!(self.len(), c, "splitting shouldn't change the size");
        assert!(
            (new_size <= old_size + other_size) && (new_size + other_size >= old_size),
            "new cubea re either too big or too small"
        );

        res
    }

    fn contains(&self, p: &(i32, i32, i32)) -> bool {
        let (x, y, z) = *p;
        self.x_min <= x
            && x <= self.x_max
            && self.y_min <= y
            && y <= self.y_max
            && self.z_min <= z
            && z <= self.z_max
    }
}
/*
const INPUT_DATA: &str = include_str!("input.txt");
*/

fn to_range(r: &str) -> impl Iterator<Item = i32> {
    let r = &r[2..];
    let (start, end) = r
        .split("..")
        .map(|x| x.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();
    return start..=end;
}

fn to_tuple(r: &str) -> (i32, i32) {
    let r = &r[2..];
    let (start, end) = r
        .split("..")
        .map(|x| x.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();
    return (start, end);
}
fn new_from_file(content: &str) -> Vec<Cube> {
    let lines = content.split("\n");

    let mut result: Vec<Cube> = Vec::new();

    for line in lines {
        let (ins, line) = line.split_once(" ").unwrap();
        let add = ins == "on";
        let (xp, yp, zp) = line.split(",").map(to_tuple).collect_tuple().unwrap();
        let cube = Cube::new_from_tuples(xp, yp, zp);
        let mut n_result = Vec::new();

        for existing in result {
            for piece in existing.subtract(&cube) {
                if !piece.overlaps(&cube) {
                    n_result.push(piece);
                }
            }
        }
        if add {
            n_result.push(cube);
        }

        Cube::assert_no_overlapping(&n_result);
        result = n_result;
    }

    result
}
fn main() {
    const INPUT_DATA: &str = include_str!("input.txt");
    let result = new_from_file(INPUT_DATA);

    let c: usize = result.iter().map(Cube::len).sum();
    println!("Part1 {:?}", c);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = include_str!("test.txt");

    const TEST2_DATA: &str = include_str!("test2.txt");
    #[test]
    fn test_prt1() {
        let result = new_from_file(TEST_DATA);
        println!("test1 {:?}", result);
        let c: usize = result.iter().map(Cube::len).sum();
        assert_eq!(c, 39)
    }
    #[test]
    fn test2_prt1() {
        let result = new_from_file(TEST2_DATA);

        let c: usize = result.iter().map(Cube::len).sum();
        assert_eq!(c, 590784);
    }

    #[test]
    fn input_prt1() {
        const INPUT_DATA: &str = include_str!("input.txt");
        let result = new_from_file(INPUT_DATA);

        let c: usize = result.iter().map(Cube::len).sum();
        assert_eq!(c, 527915);
    }

    #[test]
    fn test3_part2() {
        const TEST3_DATA: &str = include_str!("test3.txt");
        let result = new_from_file(TEST3_DATA);

        let c: usize = result.iter().map(Cube::len).sum();
        assert_eq!(c, 2758514936282235);
    }
}
