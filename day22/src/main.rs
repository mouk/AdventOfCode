use std::collections::HashSet;

use itertools::*;
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
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
        //println!("Split case 1");
        result.push((x_min, x2_min - 1));
        if x2_max < x_max {
            result.push((x2_min, x2_max));
            result.push((x2_max + 1, x_max));
        } else {
            result.push((x2_min, x_max));
        }
    } else if x_min <= x2_max && x2_max < x_max {
        //println!("Split case 2");
        result.push((x_min, x2_max));
        result.push((x2_max + 1, x_max));
    } else {
        //println!("Split case 3");
        result.push((x_min, x_max));
    }

    //println!("Split {:?} {:?} BY {:?} {:?}", x_min, x_max, x2_min, x2_max);
    //println!("Result {:?}", result);
    assert_eq!(result.last().unwrap().1, x_max, "x max");
    assert_eq!(result[0].0, x_min, "x min");

    for i in 1..result.len() {
        assert_eq!(result[i - 1].1 + 1, result[i].0)
    }

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
        [
            (self.x_min, self.y_min, self.z_min),
            (self.x_max, self.y_min, self.z_min),
            (self.x_max, self.y_max, self.z_min),
            (self.x_max, self.y_max, self.z_max),
            (self.x_max, self.y_min, self.z_max),
            (self.x_min, self.y_max, self.z_min),
            (self.x_min, self.y_max, self.z_max),
            (self.x_min, self.y_min, self.z_max),
        ]
    }

    fn new_from_tuples(xp: (i32, i32), yp: (i32, i32), zp: (i32, i32)) -> Self {
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
        }
    }

    fn new_from_line(line: &str) -> (Self, bool) {
        println!("Processing line {}", line);
        let (ins, line) = line.split_once(" ").unwrap();
        let add = ins == "on";
        let (xp, yp, zp) = line.split(",").map(to_tuple).collect_tuple().unwrap();
        let cube = Cube::new_from_tuples(xp, yp, zp);
        (cube, add)
    }

    fn assert_no_overlapping(cubes: &HashSet<Self>, message: &str) {
        /*
        let overlapping = cubes
            .iter()
            .map(|c| c.points())
            .tuple_combinations()
            .filter(|(c1, c2)| c1.intersection(&c2).count() != 0)
            .collect_HashSet();
        */
        let overlapping = cubes
            .iter()
            .tuple_combinations()
            .filter(|(c1, c2)| c1.overlaps(c2))
            .collect::<HashSet<_>>();

        if overlapping.len() != 0 {
            for c in &overlapping {
                println!("{}:Overlapping {:?} ", message, c);
            }
        }

        debug_assert!(overlapping.len() == 0, "No overlapping");
    }
    fn subtract(&self, other: &Self) -> HashSet<Self> {
        if !self.overlaps(other) {
            return HashSet::from_iter(vec![self.clone()]);
        }

        let mut res = HashSet::new();

        let mut discarded = HashSet::new();

        let x_range = split_range(self.x_min, self.x_max, other.x_min, other.x_max);
        let y_range = split_range(self.y_min, self.y_max, other.y_min, other.y_max);
        let z_range = split_range(self.z_min, self.z_max, other.z_min, other.z_max);
        //println!("X {:?} ", x_range);
        //println!("Y {:?} ", y_range);
        //println!("Z {:?} ", z_range);
        for xp in x_range {
            if other.x_max < xp.0 || xp.1 < other.x_min {
                let cube =
                    Cube::new_from_tuples(xp, (self.y_min, self.y_max), (self.z_min, self.z_max));
                res.insert(cube);
                continue;
            }
            for &yp in &y_range {
                if other.y_max < yp.0 || yp.1 < other.y_min {
                    let cube = Cube::new_from_tuples(xp, yp, (self.z_min, self.z_max));
                    res.insert(cube);
                    continue;
                }
                for &zp in &z_range {
                    let cube = Cube::new_from_tuples(xp, yp, zp);
                    if !other.overlaps(&cube) {
                        res.insert(cube);
                    } else {
                        discarded.insert(cube);
                    }
                }
            }
        }

        assert!(discarded.len() <= 1, "Only one cube can be discarded");

        let mut all_of_them = res.clone();
        for d in discarded.clone() {
            all_of_them.insert(d);
        }

        Cube::assert_no_overlapping(&all_of_them, r#"all of them"#);

        let c: usize = all_of_them.iter().map(Cube::len).sum::<usize>();

        if c != self.len() {
            println!("Original {:?}", self);
            println!("Split to {:?} items:  {:?}", res.len(), res);
        }

        assert_eq!(self.len(), c, "splitting shouldn't change the size");
        assert!(
            discarded.iter().map(Cube::len).sum::<usize>() <= other.len(),
            "discarded cannot be bigger than the subtracted part"
        );

        let not_included = all_of_them
            .iter()
            .filter(|&c| !self.contains_cube(c))
            .collect::<HashSet<_>>();

        assert!(
            not_included.len() == 0,
            "No sub cube should be outside the main cube"
        );

        res
    }

    fn contains(&self, p: &(i32, i32, i32)) -> bool {
        //return self.points().contains(p);

        let (x, y, z) = *p;
        self.x_min <= x
            && x <= self.x_max
            && self.y_min <= y
            && y <= self.y_max
            && self.z_min <= z
            && z <= self.z_max
    }

    fn points(&self) -> HashSet<(i32, i32, i32)> {
        let mut result = HashSet::new();
        for x in self.x_min..=self.x_max {
            for y in self.y_min..=self.y_max {
                for z in self.z_min..=self.z_max {
                    result.insert((x, y, z));
                }
            }
        }
        result
    }

    fn contains_cube(&self, other: &Self) -> bool {
        /*
                let my_points = self.points();
                return other.points().iter().all(|p| my_points.contains(p));
        */
        self.x_min <= other.x_min
            && other.x_max <= self.x_max
            && self.y_min <= other.y_min
            && other.y_max <= self.y_max
            && self.z_min <= other.z_min
            && other.z_max <= self.z_max
    }
}

fn to_tuple(r: &str) -> (i32, i32) {
    let r = &r[2..];
    let (start, end) = r
        .split("..")
        .map(|x| x.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();

    assert!(start <= end, "start cannot be greater than end");
    return (start, end);
}
fn new_from_file(content: &str) -> HashSet<Cube> {
    let lines = content.split("\n");

    let mut result: HashSet<Cube> = HashSet::new();

    for line in lines {
        result = apply_new_line(line, result);
    }

    result
}

fn apply_new_line(line: &str, result: HashSet<Cube>) -> HashSet<Cube> {
    Cube::assert_no_overlapping(&result, "piece.overlaps(&cube) ");
    let (new_cube, add) = Cube::new_from_line(line);
    let mut n_result = HashSet::new();
    let copy = result.clone();
    for existing in result {
        debug_assert!(
            copy.iter().all(|c| existing == *c || !existing.overlaps(c)),
            "copy already contains somethin similar 5"
        );
        debug_assert!(
            n_result.iter().all(|c| !existing.overlaps(c)),
            "n_result already contains somethin similar 5"
        );
        let subtracted = existing.subtract(&new_cube);
        Cube::assert_no_overlapping(&subtracted, "subtracted already overlapping");
        for piece in subtracted {
            //if !piece.overlaps(&new_cube) {
            debug_assert!(
                n_result.iter().all(|c| !piece.overlaps(c)),
                "n_result already contains something similar 2"
            );
            n_result.insert(piece);
            //}
        }
        Cube::assert_no_overlapping(&n_result, "piece.overlaps(&cube) ");
    }
    debug_assert!(
        n_result.iter().all(|c| !new_cube.overlaps(c)),
        "At this point no cube should overlap with the new_created"
    );
    if add {
        println!("Adding all new cubioid {:?} ", &new_cube);
        n_result.insert(new_cube);
    }
    println!(
        "Number of cubioids became {} with {} cubes",
        &n_result.len(),
        &n_result.iter().map(Cube::len).sum::<usize>()
    );
    Cube::assert_no_overlapping(&n_result, "Final");
    return n_result;
}
fn main() {
    const INPUT_DATA: &str = include_str!("input2.txt");
    let result = new_from_file(INPUT_DATA);

    let c: usize = result.iter().map(Cube::len).sum();
    println!("Part2 {:?}", c);
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
    fn input_part1() {
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
    /*
    #[test]
    fn splitrange() {
        //starts before first
        assert_eq!(split_range(10, 20, 0, 9), HashSet![(10, 20)]);

        assert_eq!(split_range(10, 20, 0, 10), HashSet![(10, 10), (11, 20)]);

        assert_eq!(split_range(10, 20, 0, 11), HashSet![(10, 11), (12, 20)]);

        assert_eq!(split_range(10, 20, 0, 20), HashSet![(10, 20)]);

        assert_eq!(split_range(10, 20, 0, 21), HashSet![(10, 20)]);
        //starts at before first

        assert_eq!(split_range(10, 20, 10, 10), HashSet![(10, 10), (11, 20)]);

        assert_eq!(split_range(10, 20, 10, 11), HashSet![(10, 11), (12, 20)]);

        assert_eq!(split_range(10, 20, 10, 20), HashSet![(10, 20)]);

        assert_eq!(split_range(10, 20, 10, 21), HashSet![(10, 20)]);

        //starts in the middle

        assert_eq!(
            split_range(10, 20, 11, 11),
            HashSet![(10, 10), (11, 11), (12, 20)]
        );

        assert_eq!(
            split_range(10, 20, 11, 12),
            HashSet![(10, 10), (11, 12), (13, 20)]
        );

        assert_eq!(split_range(10, 20, 11, 20), HashSet![(10, 10), (11, 20)]);
        assert_eq!(split_range(10, 20, 11, 21), HashSet![(10, 10), (11, 20)]);

        //starts at the end

        assert_eq!(split_range(10, 20, 20, 20), HashSet![(10, 19), (20, 20)]);
        assert_eq!(split_range(10, 20, 20, 21), HashSet![(10, 19), (20, 20)]);

        //starts beyond

        assert_eq!(split_range(10, 20, 30, 40), HashSet![(10, 20)]);
    }
    */

    #[test]
    fn apply_new_line_test1() {
        let result = HashSet::new();
        let result = apply_new_line(
            "on x=-57769..-43737,y=-43364..-27692,z=26689..48069",
            result,
        );
        let result = apply_new_line(
            "on x=-33705..-1437,y=-50304..-33797,z=-65845..-40680",
            result,
        );

        let c: usize = result.len();
        assert_eq!(c, 2);
        println!("{:?}", result.iter().last().unwrap().len())
    }
}
