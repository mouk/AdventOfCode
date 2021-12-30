use itertools::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Permutation {
    indices: [usize; 3],
    factors: [i32; 3],
}

struct Scan {
    beacons: Vec<HashSet<Pos>>,
    sensors: Vec<Vec<Pos>>,
}

impl Scan {
    /*
    fn len(&self) -> usize {

        self.beacons[0].len()
    }
     */

    fn sets_count(&self) -> usize {
        self.beacons.len()
    }

    fn sensors_count(&self) -> usize {
        self.sensors.iter().map(Vec::len).sum()
    }
}

#[derive(Debug, Clone)]
struct LinearMap {
    permutation: Permutation,
    distance: Pos,
}

type Pos = (i32, i32, i32); // [i32; 3];
const TEST_DATA: &str = include_str!("test.txt");
const INPUT_DATA: &str = include_str!("input.txt");

fn parse(text: &str) -> Scan {
    let beacons = text
        .split("\n\n")
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
        .collect::<Vec<HashSet<_>>>();
    let sensors = (0..beacons.len())
        .map(|_| vec![(0, 0, 0)])
        .collect::<Vec<Vec<Pos>>>();

    assert_eq!(
        beacons.len(),
        sensors.len(),
        "there must be as many sensors an beacons sets"
    );

    assert!(
        sensors.iter().all(|s| s.len() == 1),
        "At the beginning every set has one sensor"
    );

    Scan { sensors, beacons }
}
/*
fn distance(p1: &Pos, p2: &Pos) -> u64 {
    (((p1[0] - p2[0]).pow(2) + (p1[1] - p2[1]).pow(2) + (p1[2] - p2[2]).pow(2)) as f64).sqrt()
        as u64
}
 */
fn distance(p1: &Pos, p2: &Pos) -> u32 {
    //(p1.0 - p2.0).pow(2) as u32 + (p1.1 - p2.1).pow(2) as u32 + (p1.2 - p2.2).pow(2) as u32
    (p1.0 - p2.0).abs() as u32 + (p1.1 - p2.1).abs() as u32 + (p1.2 - p2.2).abs() as u32
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
    let (combined_result, sensors) = combine_to_one(d, 12);

    println!("Numer of distinct beacons {}", combined_result.len());

    let max_distances = sensors
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| distance(p1, p2))
        .max()
        .unwrap();

    println!("Part2 {}", max_distances);
}

fn combine_to_one(scan: Scan, min_matches: usize) -> (HashSet<Pos>, Vec<Pos>) {
    let old_count = scan.sensors_count();

    println!("old count {:?}", old_count);
    println!("starting Sensors {:?}", &scan.sensors);
    let mut new_result = scan;

    loop {
        let last_count = new_result.sets_count();
        new_result = combine(new_result, min_matches);

        println!("Sensors {:?}", new_result.sensors);
        let sensor_count: usize = new_result.sensors_count();
        assert_eq!(
            sensor_count, old_count,
            "The total number of sensor shouldn't change"
        );

        if new_result.sets_count() >= last_count || new_result.sets_count() == 1 {
            break;
        }
    }

    println!(
        "Number of distinct sets was reduced from  {} to {}",
        old_count,
        new_result.sets_count()
    );

    assert_eq!(new_result.sets_count(), 1, "Didn't finish");
    (
        new_result.beacons.pop().unwrap(),
        new_result.sensors.pop().unwrap(),
    )
}
fn combine(scan: Scan, min_matches: usize) -> Scan {
    let d = scan.beacons;
    let sensors = scan.sensors;

    let threash_hold = choose_2_count(min_matches);
    let mut merged = HashSet::new();
    let mut result = Vec::new();
    let mut result_sensors = Vec::new();
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

    let candidates = distances
        .iter()
        .enumerate()
        .tuple_combinations()
        .filter(|((_, set1), (_, set2))| {
            return set1.intersection(set2).count() >= threash_hold;
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

                let intersection = mapped_set1.intersection(&d[can2]);

                if intersection.count() >= min_matches {
                    println!("ADDING {:?} and  {:?} to merged", can1, can2);
                    merged.insert(can1);
                    merged.insert(can2);
                    result.push(
                        mapped_set1
                            .union(&d[can2])
                            .map(|&d| d)
                            .collect::<HashSet<Pos>>(),
                    );

                    let mut mapped_sensors = sensors[can1]
                        .clone()
                        .iter()
                        .map(|p| apply_linear_map(&perm, p))
                        .collect_vec();
                    mapped_sensors.append(&mut sensors[can2].clone());
                    result_sensors.push(mapped_sensors);

                    break 'outer;
                } else {
                    println!("Intersection is too small")
                }
            }
        }
    }

    for i in 0..(d.len()) {
        if !merged.contains(&i) {
            result.push(d[i].clone());
            result_sensors.push(sensors[i].clone());
        }
    }

    assert_eq!(result.len(), result_sensors.len());

    assert_eq!(
        sensors.iter().map(Vec::len).sum::<usize>(),
        result_sensors.iter().map(Vec::len).sum::<usize>(),
        "Number of sensor should remain unchanged"
    );
    Scan {
        beacons: result,
        sensors: result_sensors,
    }
}

fn choose_2_count(set_count: usize) -> usize {
    (set_count * (set_count - 1)) / 2
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
        let m = diff(&p1_mapped, p2);
        let q1_mapped = apply(&perm, q1);
        let m2 = diff(&q1_mapped, q2);
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

// #[test]
fn test_try_map() {
    let f = (&(1, 2, 3), &(2, 3, 4));

    let s = (&(2, 3, 4), &(3, 4, 5));

    for perm in try_map(f, s) {
        assert_eq!(apply_linear_map(&perm, f.0), *s.0);
    }
}

#[test]
fn test_input_part1() {
    let d = parse(TEST_DATA);
    let (combined_result, _) = combine_to_one(d, 5);

    assert_eq!(combined_result.len(), 79);
}

#[test]
fn input_part1() {
    let d = parse(INPUT_DATA);
    let (combined_result, _) = combine_to_one(d, 12);

    assert_eq!(combined_result.len(), 359);
}
