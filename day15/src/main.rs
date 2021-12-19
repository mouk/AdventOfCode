use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;
use std::path::Path;

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

// Each node is represented as a `usize`, for a shorter implementation.
#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

struct RiskMap {
    data: Vec<usize>,
    dimension: usize,
}
fn wrap(num: usize) -> usize {
    return ((num - 1) % 9) + 1;
}
impl RiskMap {
    fn expand(&self, factor: usize) -> Self {
        let dim = self.dimension;
        let new_dim = dim * factor;
        let new_size = self.len() * factor * factor;
        let mut new_data = vec![0; new_size];

        //one row
        for row in 0..dim {
            for i in 0..factor {
                //new row i
                for x in 0..dim {
                    new_data[x + (i * dim) + (row * new_dim)] =
                        wrap(self.data[x + (row * dim)] + i);
                }
            }
        }

        //expand rows
        for row in 1..factor {
            for line in 0..dim {
                for col in 0..new_dim {
                    new_data[col + line * new_dim + row * (dim * new_dim)] =
                        wrap(new_data[col + line * new_dim] + row);
                }
            }
        }

        return RiskMap {
            data: new_data,
            dimension: new_dim,
        };
    }

    fn print(&self) -> String {
        let mut s = String::new();
        for i in 0..self.len() {
            s.push_str(&self.data[i].to_string());
            if (i + 1) % self.dimension == 0 {
                s.push_str("\n");
            }
        }
        s.to_string()
    }
    fn get_input<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let content: Vec<u8> = fs::read(path).expect("war richtig");

        Self::new_from_file_content(content)
    }

    fn new_from_file_content(content: Vec<u8>) -> Self {
        let data: Vec<usize> = content
            .iter()
            .filter(|&&x| x >= 48 && x <= 57)
            .cloned()
            .map(|c| usize::from(c - 48))
            .collect();

        let dim = ((&data.len() + 1) as f64).sqrt() as usize;
        return RiskMap {
            data: data,
            dimension: dim,
        };
    }

    fn len(&self) -> usize {
        self.data.len()
    }
    fn get_adj(&self) -> Vec<Vec<Edge>> {
        (00..(self.data.len()))
            .collect::<Vec<usize>>()
            .iter()
            .map(|l| self.get_neighbors(*l))
            .collect::<Vec<Vec<Edge>>>()
    }
    fn get_neighbors(&self, index: usize) -> Vec<Edge> {
        let mut result: Vec<_> = Vec::new();
        let dim = self.dimension;
        //north
        if index >= dim {
            let i = index - dim;
            result.push(Edge {
                node: i,
                cost: self.data[i],
            });
        }
        //east
        if (index + 1) % dim != 0 {
            let i = index + 1;
            result.push(Edge {
                node: i,
                cost: self.data[i],
            });
        }
        //south
        if (index + dim) < self.data.len() {
            let i = index + dim;
            result.push(Edge {
                node: i,
                cost: self.data[i],
            });
        }
        //west
        if index % dim != 0 {
            let i = index - 1;
            result.push(Edge {
                node: i,
                cost: self.data[i],
            });
        }
        return result;
    }
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
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
                cost: cost + edge.cost,
                position: edge.node,
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

fn main() {
    println!("Hello, world!");
    let content: Vec<u8> = fs::read("./src/input.txt").expect("war richtig");

    let heat_map = RiskMap::new_from_file_content(content);

    //println!("adj {:?}", &heat_map.get_adj());
    let path = shortest_path(&heat_map.get_adj(), 0, heat_map.len() - 1);

    println!("Tes {:?}", path);

    let new_map = heat_map.expand(5);
    //println!("adj {:?}", &heat_map.get_adj());
    let path = shortest_path(&new_map.get_adj(), 0, new_map.len() - 1);

    println!("Part2 {:?}", path);
    //println!("{}", new_map.print());
}

#[test]
fn test_data_simple() {
    let heat_map = RiskMap::get_input("src/test.txt");
    let path = shortest_path(&heat_map.get_adj(), 0, heat_map.len() - 1);

    assert_eq!(path, Some(40));
}

#[test]
fn test_data_extendd() {
    let heat_map = RiskMap::get_input("src/test.txt");
    let heat_map = heat_map.expand(5);
    let path = shortest_path(&heat_map.get_adj(), 0, heat_map.len() - 1);

    assert_eq!(path, Some(315));
}
#[test]
fn input_data_simple() {
    let heat_map = RiskMap::get_input("src/input.txt");
    let path = shortest_path(&heat_map.get_adj(), 0, heat_map.len() - 1);

    assert_eq!(path, Some(423));
}
