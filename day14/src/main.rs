use counter::Counter;
use std::{collections::HashMap, fs, path::Path}; // 0.

type Template = HashMap<(char, char), usize>;
type Insertions = HashMap<(char, char), [(char, char); 2]>;
#[derive(Debug)]
struct Polymerization {
    template: Template,
    //insertions: HashMap<String, String>,
    start: char,
    end: char,
}
impl Polymerization {
    fn add_years(&mut self, ins: &Insertions, years: usize) {
        for _ in 0..years {
            self.add_year(ins);
        }
    }

    fn add_year(&mut self, ins: &Insertions) {
        let n_gen = self
            .template
            .iter()
            .map(|(key, size)| match ins.get(key) {
                Some(a) => [(a[0], *size), (a[1], *size)],
                None => panic!("not possible"), //[(*key, *size)],
            })
            .flatten();
        let mut template: HashMap<(char, char), usize> = HashMap::new();
        for (key, count) in n_gen {
            let entry = template.entry(key).or_insert(0);
            *entry += count;
        }
        self.template = template;
    }

    fn get_diff(&self) -> usize {
        let mut map: HashMap<char, usize> = HashMap::new();

        for ((a, b), count) in &self.template {
            let e = map.entry(*a).or_insert(0);
            *e += *count;
            let e = map.entry(*b).or_insert(0);
            *e += *count;
        }
        //remove start and end
        let e = map.entry(self.start).or_insert(0);
        *e += 1;
        let e = map.entry(self.end).or_insert(0);
        *e += 1;

        //println!("map {:?}", map);

        let max = map.values().max().unwrap();
        let min = map.values().min().unwrap();
        (*max - *min) / 2
    }

    fn len(&self) -> usize {
        self.template.values().sum::<usize>() + 1
    }
}

fn get_input<P>(path: P) -> (Polymerization, Insertions)
where
    P: AsRef<Path>,
{
    let content = fs::read_to_string(path).expect("invalid");

    let (template, rules) = content.split_once("\n\n").unwrap();

    let start = template.chars().nth(0).unwrap();
    let end = template.chars().last().unwrap();
    let template = template
        .chars()
        .zip(template.chars().skip(1))
        .collect::<Counter<_>>()
        .into_map();

    let poly = Polymerization {
        template,
        start,
        end,
    };
    let pairs = rules.split("\n").map(|line| {
        let (left, right) = line.split_once(" -> ").unwrap();
        let mut lefts = left.chars();
        let left1 = lefts.next().unwrap();
        let left2 = lefts.next().unwrap();
        let right = right.chars().next().unwrap();

        ((left1, left2), [(left1, right), (right, left2)])
    });

    let insertions = HashMap::from_iter(pairs);
    (poly, insertions)
}

fn main() {
    let (mut poly, insertions) = get_input("src/input.txt");

    poly.add_years(&insertions, 10);
    println!("Part1 {:?}", poly.get_diff());
    poly.add_years(&insertions, 30);
    println!("Part2 {:?}", poly.get_diff());
}

#[test]
fn test_data_diff() {
    let (mut poly, insertions) = get_input("src/test.txt");
    poly.add_years(&insertions, 10);
    assert_eq!(poly.get_diff(), 1588);
}

#[test]
fn test_data_length() {
    let (mut poly, insertions) = get_input("src/test.txt");

    poly.add_years(&insertions, 5);

    assert_eq!(poly.len(), 97);
}

#[test]
fn input_data_part1() {
    let (mut poly, insertions) = get_input("src/input.txt");

    poly.add_years(&insertions, 10);

    assert_eq!(poly.get_diff(), 2975);
}

#[test]
fn test_data_part1() {
    let (mut poly, insertions) = get_input("src/test.txt");

    poly.add_years(&insertions, 10);
    assert_eq!(poly.get_diff(), 1588);
}

#[test]
fn test_data_part2() {
    let (mut poly, insertions) = get_input("src/test.txt");
    poly.add_years(&insertions, 40);
    assert_eq!(poly.get_diff(), 2188189693529);
}
