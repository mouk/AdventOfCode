use itertools::*;
use std::collections::HashSet;

//const INPUT_DATA: &str = include_str!("input.txt");

fn to_range(r: &str) -> impl Iterator<Item = i32> {
    let r = &r[2..];
    let (start, end) = r
        .split("..")
        .map(|x| x.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();
    return start..=end;
}
fn new_from_file(content: &str) -> HashSet<(i32, i32, i32)> {
    let lines = content.split("\n");

    let mut result = HashSet::new();

    for line in lines {
        let (ins, line) = line.split_once(" ").unwrap();
        let add = ins == "on";
        let (x, y, z) = line.split(",").collect_tuple().unwrap();
        for xi in to_range(x) {
            for yi in to_range(x) {
                for zi in to_range(x) {
                    if add {
                        result.insert((xi, yi, zi));
                    } else {
                        result.remove(&(xi, yi, zi));
                    }
                }
            }
        }
    }

    result
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = include_str!("test.txt");

    const TEST2_DATA: &str = include_str!("test2.txt");
    #[test]
    fn test_prt1() {
        let result = new_from_file(TEST_DATA);
        assert_eq!(result.len(), 39)
    }
    #[test]
    fn test2_prt1() {
        let result = new_from_file(TEST2_DATA);
        assert_eq!(result.len(), 590784)
    }
}
