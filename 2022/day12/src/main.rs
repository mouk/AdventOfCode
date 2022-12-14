use std::{collections::BinaryHeap, cmp::Ordering, ops::Index};

#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: usize,
}


#[derive(Clone, Debug)]
struct RiskMap {
    start: usize,
    target: usize,
    data: Vec<usize>,
    width: usize,
    height: usize
}

impl RiskMap {
   fn print(&self) -> String {
        let mut s = String::new();
        for i in 0..self.len() {
           
            let c = if  i == self.start {'S'} else if i == self.target {'E'} else {char::from_u32(97 + self.data[i] as u32).unwrap()};
            s.push_str(&c.to_string());
            s.push_str(" ");
            if (i + 1) % self.width == 0 {
                s.push_str("\n");
            }
        }
        s.to_string()
    }

    fn new_from_file_content(content: &str) -> Self {

        let mut start = 1000;
        let mut target = 10000;

        let width = content.find('\n').unwrap();

        let data: Vec<usize> = content
        .chars()
        .filter(|&x| x != '\n')
        .enumerate()
        .map(|(index,c)|{
            if c == 'S' {
                start = index;
                return 'a';
            }else if c == 'E'{
                target = index;
                return 'z'; 
            }
            c
        })
        .map(|c| c as usize - 97)
        .collect();

        let height = data.len()/width;

        println!("count: {}, width: {}, height {}",data.len(),  width, height);
        RiskMap {
            start,
            target,
            data,
            width,
            height
        }
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
        let w = self.width;
        //north
        if index >= w {
            let i = index - w;
            result.push(i);
        }
        //east
        if (index + 1) % w != 0 {
            let i = index + 1;
            result.push(i);
        }
        //south
        if (index + w) < self.data.len() {
            let i = index + w;
            result.push(i);
        }
        //west
        if index % w != 0 {
            let i = index - 1;
            result.push(i);
        }
        let current_signal = self.data[index];

      
        result
        .iter()
        .filter(|&i| self.data[*i] < 2 + current_signal)
        .map(|e| Edge {
            node: *e,
            cost: 1,
        })
        .collect()
    }
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
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| 100).collect();

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
            let mut s = String::new();
            for i in 0..dist.len() {
                s.push_str(&dist[i].to_string());
                s.push_str("\t");
                if (i + 1) % 96 == 0 {
                    s.push_str("\n");
                }
            }
            
            println!("{}", s.to_string());
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

    const INPUT:&str  = include_str!("input.txt");
    let m = RiskMap::new_from_file_content(INPUT);
    println!("{}", &m.print());
    let costs = shortest_path(&m.get_adj(), m.start, m.target);
    println!("Hello, world! {:?}", costs);
}
