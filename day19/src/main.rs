use itertools::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Permutation {
    indices: [usize; 3],
    factors: [i32; 3],
}

#[derive(Debug, Clone)]
struct LinearMap {
    permutation: Permutation,
    distance: Pos,
}

type Pos = (i32, i32, i32); // [i32; 3];
const TEST_DATA: &str = include_str!("test.txt");
const INPUT_DATA: &str = include_str!("input.txt");
const MIN_MATCHES_COUNT: usize = 12;
const THREAD_HOLD: usize = (MIN_MATCHES_COUNT * (MIN_MATCHES_COUNT - 1)) / 2;

fn parse(text: &str) -> Vec<HashSet<Pos>> {
    text.split("\n\n")
        .into_iter()
        .map(|chunk| {
            chunk
                .split("\n")
                .skip(1)
                .map(|l| {
                    l.split(",")
                        .map(|c| c.parse::<i32>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<HashSet<_>>>()
}
/*
fn distance(p1: &Pos, p2: &Pos) -> u64 {
    (((p1[0] - p2[0]).pow(2) + (p1[1] - p2[1]).pow(2) + (p1[2] - p2[2]).pow(2)) as f64).sqrt()
        as u64
}
 */
fn distance(p1: &Pos, p2: &Pos) -> u32 {
    (p1.0 - p2.0).pow(2) as u32 + (p1.1 - p2.1).pow(2) as u32 + (p1.2 - p2.2).pow(2) as u32
}

fn all_permutations() -> Vec<Permutation> {
    let perm = (0..3).permutations(3).map(|a| [a[0], a[1], a[2]]); // .collect::<Vec<_>>();
    let d = [1, -1];
    let factors = d
        .iter()
        .cartesian_product(d)
        .cartesian_product(d)
        .map(|((&x, y), z)| [x, y, z])
        .collect::<Vec<_>>();

    perm.cartesian_product(factors)
        .map(|(indices, factors)| Permutation { indices, factors })
        .collect::<Vec<_>>()
}

fn main() {
    let d = parse(INPUT_DATA);
    let old_count = d.len();
    let mut last_count = old_count;
    let mut new_result = combine(d);

    while new_result.len() < last_count && new_result.len() > 1 {
        last_count = new_result.len();
        new_result = combine(new_result);
    }

    assert_eq!(new_result.len(), 1);
    println!(
        "Number of distinct sets was reduced from  {} to {}",
        old_count,
        new_result.len()
    );

    println!("Numer of distinct beacons {}", new_result[0].len());
}
fn combine(d: Vec<HashSet<Pos>>) -> Vec<HashSet<Pos>> {
    let mut merged = HashSet::new();
    let mut result = Vec::new();
    let distances = d
        .iter()
        .map(|sensor| {
            sensor
                .iter()
                .tuple_combinations()
                .map(|(p1, p2)| distance(p1, p2))
                .collect::<HashSet<u32>>()
        })
        .collect::<Vec<_>>();

    println!("distances {:?} {:?}", distances, distances[0].len());

    let candidates = distances
        .iter()
        .enumerate()
        .tuple_combinations()
        .filter(|((_, set1), (_, set2))| {
            return set1.intersection(set2).count() >= THREAD_HOLD;
        })
        .map(|((x, _), (y, _))| (x, y))
        .collect::<Vec<(usize, usize)>>();

    //let alligned = vec![vec![]];

    for (can1, can2) in candidates {
        if merged.contains(&can1) || merged.contains(&can2) {
            continue;
        }
        println!("Comparing sets number  {:?} and {:?}", can1, can2);
        let mut intersection = distances[can1].intersection(&distances[can2]).collect_vec();

        'outer: while let Some(&a_common_distance) = intersection.pop() {
            let first_pair = d[can1]
                .iter()
                .tuple_combinations()
                .find(|(p1, p2)| distance(p1, p2) == a_common_distance)
                .unwrap();

            let second_pair = d[can2]
                .iter()
                .tuple_combinations()
                .find(|(p1, p2)| distance(p1, p2) == a_common_distance)
                .unwrap();

            let transformations = try_map(first_pair, second_pair);

            for perm in transformations {
                let mapped_set1 = d[can1]
                    .iter()
                    .map(|p| apply_linear_map(&perm, p))
                    .collect::<HashSet<Pos>>();

                let res = mapped_set1.intersection(&d[can2]);

                let count = res.count();
                if count >= MIN_MATCHES_COUNT {
                    println!("ADDING {:?} and  {:?} to merged", can1, can2);
                    merged.insert(can1);

                    merged.insert(can2);
                    result.push(
                        mapped_set1
                            .union(&d[can2])
                            .map(|&d| d)
                            .collect::<HashSet<Pos>>(),
                    );
                    break 'outer;
                } else {
                    println!("Intersection is too small, {}", count)
                }
            }
        }
    }

    for i in 0..(d.len()) {
        if !merged.contains(&i) {
            result.push(d[i].clone());
        }
    }
    result
}

fn try_map(
    first_pair: (&(i32, i32, i32), &(i32, i32, i32)),
    second_pair: (&(i32, i32, i32), &(i32, i32, i32)),
) -> Vec<LinearMap> {
    let mut result = Vec::new();
    let perms = all_permutations();
    let (p1, q1) = first_pair;
    let (p2, q2) = second_pair;
    for perm in perms {
        //try mapping p1 to p2
        let p1_mapped = apply(&perm, p1);
        //println!("P1 is {:?} and mapped {:?}", p1, p1_mapped);
        let m = diff(&p1_mapped, p2);

        //println!("m of {:?} and  {:?} is  {:?}", p1_mapped, p2, m);

        //verify by re-applying to q1 - q2
        let q1_mapped = apply(&perm, q1);
        let m2 = diff(&q1_mapped, q2);

        //println!("m of {:?} and  {:?} is  {:?}", q1_mapped, q2, m2);
        //println!("m of {:?} and  {:?} is  {:?}", q1_mapped, q2, m2);
        //println!("m of {:?} and  {:?} is  {:?}", q1_mapped, q2, m2);
        if m == m2 {
            result.push(LinearMap {
                permutation: perm,
                distance: m,
            });
            continue;
        }

        //try mapping p1 to q2
        let m = diff(&p1_mapped, q2);

        //verify by re-applying to q1 - p2
        let m2 = diff(&q1_mapped, p2);
        if m == m2 {
            result.push(LinearMap {
                permutation: perm,
                distance: m,
            });
        }
    }
    result
}

fn apply(perm: &Permutation, (x, y, z): &Pos) -> Pos {
    let arr = [*x, *y, *z];
    (0..3)
        .map(|i| arr[perm.indices[i]] * perm.factors[i])
        .collect_tuple()
        .unwrap()
}
fn diff((x, y, z): &Pos, (x2, y2, z3): &Pos) -> Pos {
    (x2 - x, y2 - y, z3 - z)
}

fn apply_linear_map(map: &LinearMap, p: &Pos) -> Pos {
    let (mx, my, mz) = apply(&map.permutation, p);
    let (x, y, z) = map.distance;
    (mx + x, my + y, mz + z)
}

#[test]
fn test_try_map() {
    let f = (&(1, 2, 3), &(2, 3, 4));

    let s = (&(2, 3, 4), &(3, 4, 5));

    for perm in try_map(f, s) {
        assert_eq!(apply_linear_map(&perm, f.0), *s.0);
    }
}
