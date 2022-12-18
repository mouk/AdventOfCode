use itertools::{Itertools, MinMaxResult};
use std::{collections::HashSet, fmt};
const TEST_DATA: &str = include_str!("test.txt");
const INPUT_DATA: &str = include_str!("input.txt");
struct Topology {
    data: HashSet<(usize, usize)>,
    max_row: usize,
}

impl Topology {
    fn from_text(input: &str) -> Self {
        let data: HashSet<(usize, usize)> = input
            .split('\n')
            .map(|line| {
                line.split(" -> ")
                    .tuple_windows()
                    .map(|(start, end)| {
                        let (x1, y1) = start
                            .split(',')
                            .map(|d| d.parse::<usize>().unwrap())
                            .next_tuple()
                            .unwrap();
                        let (x2, y2) = end
                            .split(',')
                            .map(|d| d.parse::<usize>().unwrap())
                            .next_tuple()
                            .unwrap();

                        (y1.min(y2)..=y1.max(y2))
                            .cartesian_product(x1.min(x2)..=x1.max(x2))
                            .collect::<Vec<_>>()
                    })
                    .flatten()
            })
            .flatten()
            .collect();
        let max_row = data.iter().map(|(row, _)| row).max().unwrap();

        Self {
            max_row: *max_row,
            data,
        }
    }

    fn get_next(&self, p: &(usize, usize)) -> Option<(usize, usize)> {
        let (r, c) = *p;
        let r = r + 1;

        if let None = self.data.get(&(r, c)) {
            return Some((r, c));
        }
        if let None = self.data.get(&(r, c - 1)) {
            return Some((r, c - 1));
        }
        if let None = self.data.get(&(r, c + 1)) {
            return Some((r, c + 1));
        }
        None
    }

    fn fill_traditional(&mut self) -> usize {
        for i in 0..usize::MAX {
            let mut point = (0, 500);
            while let Some((r, c)) = self.get_next(&point) {
                point = (r, c);
                if r > self.max_row + 2 {
                    return i;
                }
            }
            self.data.insert(point);
            if point.0 == 0 {
                return i;
            }
        }
        0
    }


    fn fill_with_border(&mut self) -> usize {
        let floor = self.max_row + 2;
        let floor_blocks = (0..=1100)
        .map(|c| (floor,c));
        self.data.extend(floor_blocks);
        self.fill_traditional() + 1
    }
}

impl fmt::Debug for Topology {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (&min_col, &max_col) = match self.data.iter().map(|(_, col)| col).minmax() {
            MinMaxResult::MinMax(min, max) => (min, max),
            MinMaxResult::OneElement(o) => (o, o),
            _ => panic!(),
        };
        let (&min_row, &max_row) = match self.data.iter().map(|(row, _)| row).minmax() {
            MinMaxResult::MinMax(min, max) => (min, max),
            MinMaxResult::OneElement(o) => (o, o),
            _ => panic!(),
        };

        writeln!(f, "Min col {:?}, max col {:?}", min_col, max_col)?;
        writeln!(f, "Min row {:?}, max row {:?}", min_row, max_row)?;
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        for row in 0..=max_row {
            write!(f, "{}\t", row)?;

            for col in min_col..=max_col {
                match self.data.get(&(row, col)) {
                    None => write!(f, ".")?,
                    _ => write!(f, "#")?,
                }
            }
            writeln!(f, "")?;
        }
        write!(f, "")
    }
}

fn main() {

    let mut t = Topology::from_text(INPUT_DATA);

    println!("Part 1 {}", t.fill_traditional());

    let mut t = Topology::from_text(INPUT_DATA);

    println!("Part 2 {}", t.fill_with_border());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut t = Topology::from_text(INPUT_DATA);
        assert_eq!(t.fill_traditional(), 672)
    }

    #[test]
    fn test() {
        let mut t = Topology::from_text(TEST_DATA);
        assert_eq!(t.fill_traditional(), 24)
    }

    #[test]
    fn test_part2() {
        let mut t = Topology::from_text(TEST_DATA);
        assert_eq!(t.fill_with_border(), 93)
    }
}
