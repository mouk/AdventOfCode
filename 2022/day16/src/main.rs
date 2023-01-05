use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

const TEST_DATA: &str = include_str!("test.txt");
const INPUT_DATA: &str = include_str!("input.txt");

#[derive(Debug)]
struct Valve {
    flow_rate: usize,
    next: Vec<String>,
}
#[derive(Debug)]
struct Map {
    start: usize,
    flow_rates: Vec<usize>,
    shortest: Vec<usize>,
}
impl Map {
    fn from_test(input: &str) -> Self {
        let start = "AA".to_owned();
        let re: regex::Regex = Regex::new(
            r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)",
        )
        .unwrap();
        let mut tunnels = HashMap::new();
        
        for cap in re.captures_iter(input) {
            let name = cap[1].to_string();
            let flow_rate = cap[2].parse().unwrap();
            let next = cap[3]
                .to_string()
                .split(", ")
                .map(|f| f.to_owned())
                .collect();
            tunnels.insert(name, Valve { flow_rate, next });
        }

        let nodes: Vec<String> = tunnels.iter().map(|(name, _)| name.to_owned()).collect();

        let name2index = nodes
            .iter()
            .enumerate()
            .map(|(f, s)| (s.as_str(), f))
            .collect::<HashMap<_, _>>();

        let adj = nodes
            .iter()
            .map(|n| {
                tunnels
                    .get(n)
                    .unwrap()
                    .next
                    .iter()
                    .map(|to| *name2index.get(to.as_str()).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let index2name = tunnels
            .iter()
            .filter(|t| t.0 == &start || t.1.flow_rate > 0)
            .map(|f| f.0.clone())
            .enumerate()
            //.map(|(f, s)| (s.as_str(), f))
            .collect::<HashMap<_, _>>();

        let cleaned_name2index = index2name
            .iter()
            .map(|(a, b)| (b.clone(), *a))
            .collect::<HashMap<_, _>>();

            let len = cleaned_name2index.len();

        let shortest = tunnels
            .iter()
            .filter(|t| t.0 == &start || t.1.flow_rate > 0)
            .map(|f| f.0)
            .tuple_combinations()
            .map(|(start, target)| {
                let path = shortest_path(
                    &adj,
                    *name2index.get(start.as_str()).unwrap(),
                    *name2index.get(target.as_str()).unwrap(),
                );

                let path2 = shortest_path(
                    &adj,
                    *name2index.get(target.as_str()).unwrap(),
                    *name2index.get(start.as_str()).unwrap(),
                );
                return vec![
                    (
                        (start.to_owned(), target.to_owned()),
                        path.unwrap_or(usize::MAX),
                    ),
                    (
                        (target.to_owned(), start.to_owned()),
                        path2.unwrap_or(usize::MAX),
                    ),
                ];
            })
            .flatten()
            .map(|((f, s), u)| {
                (
                    (
                        *cleaned_name2index.get(&f).unwrap() * len +
                        *cleaned_name2index.get(&s).unwrap()
                    ),
                    u
                )
            })
            .sorted_by_key(|p| p.0)
            .collect::<HashMap<_,_>>();

            let shortest =(00..(len * len))
            .map(|i| *shortest.get(&i).unwrap_or(&0) )
            .collect::<Vec< _>>();

        let flow_rates = tunnels
            .iter()
            .filter(|t| t.0 == &start || t.1.flow_rate > 0)
            .map(|(name, valve)| (cleaned_name2index.get(name).unwrap(), valve.flow_rate))
            .sorted_by(|a, b| Ord::cmp(a.0, b.0))
            .map(|(_, valve)| valve)
            .collect();

        Map {
            start: *cleaned_name2index.get("AA").unwrap(),
            flow_rates,
            shortest,
        }
    }

    
    fn calculate_with_elephant(&self) -> usize {
        let len = self.flow_rates.len() - 1;
        let max = (1 << len)-1 as usize;
        (0..max)
        .into_par_iter()
        //.filter(|mask| *mask < !*mask)
        .map(|m|  self.calculate_mask(m, 26) + self.calculate_mask(!m , 26) )
        .max()
        .unwrap()
    }
    fn calculate(&self) -> usize {
        self.calculate_mask(usize::MAX,30)
    }
    fn calculate_mask(&self, nodes_mask: usize, remaining: usize) -> usize {
        let mut heap = BinaryHeap::new();

        heap.push((self.start, !nodes_mask, remaining, 0));

        let mut winner_total_flow = 0;
        let mut winner_remaining = remaining;
        let len = self.flow_rates.len();

        while let Some((current, visited , remaining, total_flow)) = heap.pop() {

            let new_visited = visited | (1 << current);
            //println!("{:b} with {:b} to. {:b}", visited, (1 << current), new_visited);

            // Just a simple if statement to check if we are on the winning node.
            if total_flow > winner_total_flow {
                //println!("winner flow {:?}", new_visited);
                winner_total_flow = total_flow;
                winner_remaining = remaining
            }

            for next in 0..self.flow_rates.len() {

                if (new_visited & (1 << next)) != 0 {
                    continue;
                }
                if let Some(&path) = self.shortest.get(current * len + next) {
                    if path >= remaining {
                        //println!("conting {}", next);
                        continue;
                    }

                    let new_remaining = remaining - path - 1;
                    let new_total_flow =
                        total_flow + (self.flow_rates.get(next).unwrap() * new_remaining);

                    //println!("pushing {}", next);

                    if new_total_flow < winner_total_flow && new_remaining < winner_remaining {
                        continue;
                    }
                    heap.push((
                        next,
                        new_visited,
                        new_remaining,
                        new_total_flow,
                    ));
                } else {
                    //println!("{:?} not found", (current, next))
                }
            }
        }
        winner_total_flow
    }
}

fn main() {
    let result = Map::from_test(INPUT_DATA);
    //println!("{:?}", result);
    //println!("Part 1 {:?}", result.calculate());
    println!("Max pressure {:?}", result.calculate_with_elephant());
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<usize>>, start: usize, goal: usize) -> Option<usize> {
    if start == goal {
        return Some(0);
    }
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + 1,
                position: *edge,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let result = Map::from_test(TEST_DATA);
        assert_eq!(result.calculate(), 1651)
    }

    #[test]
    fn test_input() {
        let result = Map::from_test(INPUT_DATA);
        assert_eq!(result.calculate(), 1944)
    }

    #[test]
    fn test_part2_input() {
        let result = Map::from_test(INPUT_DATA);
        assert_eq!(result.calculate_with_elephant(), 2679)
    }

    
}
