use core::panic;
use indextree::*;
use std::{
    cmp::max, fmt::Display, fs, iter::Filter, ops::Index, path::Path, result, slice::SliceIndex,
};
#[derive(Debug)]
struct SnailfishNumber {
    arena: Arena<Option<usize>>,
    start: NodeId,
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.fmt_from(f, self.start);
        write!(f, "{}", s)
    }
}
impl SnailfishNumber {
    fn is_pair(&self, node: NodeId) -> bool {
        (*self.arena.get(node).unwrap().get() == None)
            && node
                .children(&self.arena)
                .map(|x| *self.arena.get(x).unwrap().get())
                .all(|n| n != None)
    }
    fn reduce(&mut self) {
        /*
        println!("Was {}", self);

        println!(
            "leafs {:?}",
            self.leafs()
                .iter()
                .map(|l| self.arena.get(*l).unwrap().get())
                .collect::<Vec<_>>()
        );
        */
        loop {
            //let arena = &mut self.arena;
            let deep = self.get_first_deep(self.start, 0);
            if let Some(deep_node_id) = deep {
                let mut children = deep_node_id.children(&self.arena);
                let first = children.nth(0).unwrap();
                let last = children.last().unwrap();
                let leafs = &self.leafs();

                //println!("looking for  {:?} AND {:?}", first, last);
                //println!("leafs {:?}", leafs);

                let search_result = leafs.iter().position(|x| x == &first);
                if let None = search_result {
                    panic!("{:?} was not found", first);
                }
                let index = search_result.unwrap();
                //println!("changing arround {}", index);

                if index > 0 {
                    let first_data = self.arena.get(first).unwrap().get().unwrap();
                    let pre_sibling = leafs[index - 1];
                    let d = self.arena.get_mut(pre_sibling).unwrap().get_mut();
                    *d = Some(d.unwrap() + first_data);
                } //

                if index + 2 < leafs.len() {
                    let last_data = self.arena.get(last).unwrap().get().unwrap();
                    let post_sibling = leafs[index + 2];
                    let d = self.arena.get_mut(post_sibling).unwrap().get_mut();
                    *d = Some(d.unwrap() + last_data);
                }

                let arena = &mut self.arena;
                first.remove_subtree(arena);
                last.remove_subtree(arena);

                let d = arena.get_mut(deep_node_id).unwrap().get_mut();
                *d = Some(0 as usize);

                //println!("reduced to {}", self);
                continue;
            }

            let leafs = self.leafs();
            let node = leafs
                .into_iter()
                .filter(|l| {
                    if let Some(n) = self.arena.get(*l).unwrap().get() {
                        if *n >= 10 {
                            return true;
                        }
                    }
                    return false;
                })
                .next();

            //println!("checking high numbers");
            if let Some(big_node_id) = node {
                let val = self.arena.get(big_node_id).unwrap().get().unwrap();
                assert!(val >= 10);
                let left_val = val / 2;
                let right_val = left_val + if val % 2 == 0 { 0 } else { 1 };

                let arena = &mut self.arena;

                let left = arena.new_node(Some(left_val));
                let right = arena.new_node(Some(right_val));

                big_node_id.append(left, arena);
                big_node_id.append(right, arena);

                let big_node = arena.get_mut(big_node_id).unwrap();
                let node_val = big_node.get_mut();

                //let arena = &mself.arena;

                *node_val = None;
                continue;
            }
            break;
        }
    }
    fn leafs(&self) -> Vec<NodeId> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        stack.push(self.start);
        while let Some(current) = stack.pop() {
            let node = self.arena.get(current).unwrap();
            if let Some(_) = node.get() {
                result.push(current);
            } else {
                stack.push(node.last_child().unwrap());
                stack.push(node.first_child().unwrap());
            }
        }
        result
    }
    fn get_first_deep(&self, current: NodeId, depth_so_far: usize) -> Option<NodeId> {
        if depth_so_far >= 4 && self.is_pair(current) {
            return Some(current);
        }
        for c in current.children(&self.arena) {
            let data = self.arena.get(c).unwrap().get();
            if *data == None {
                let too_deep = self.get_first_deep(c, depth_so_far + 1);
                if let Some(_) = too_deep {
                    return too_deep;
                }
            }
        }
        return None;
    }

    fn magnitude(&self, node: NodeId) -> u64 {
        match self.arena.get(node).unwrap().get() {
            Some(x) => *x as u64,
            _ => {
                let mags = node
                    .children(&self.arena)
                    .map(|c| self.magnitude(c))
                    .collect::<Vec<_>>();
                mags[0] * 3 + 2 * mags[1]
            }
        }
    }
    fn left_num(&self, current: NodeId) -> Option<NodeId> {
        current.following_siblings(&self.arena).last()
    }
    fn add(&mut self, other: &SnailfishNumber) {
        let arena: &mut Arena<Option<usize>> = &mut self.arena;

        let new_prent = arena.new_node(None);
        new_prent.append(self.start, arena);

        self.append(&other, other.start, new_prent);

        self.start = new_prent;
        self.reduce();
    }
    fn append(&mut self, other: &SnailfishNumber, node: NodeId, new_parent_id: NodeId) {
        if None == self.arena.get(new_parent_id) {
            panic!("new_parent_id must already exists");
        }
        let arena: &mut Arena<Option<usize>> = &mut self.arena;
        let node_data = other.arena.get(node).unwrap().get();

        let data = match node_data {
            None => None,
            Some(x) => Some(*x),
        };
        let new_node_id = arena.new_node(data);
        new_parent_id.append(new_node_id, arena);
        for c in node.children(&other.arena) {
            self.append(other, c, new_node_id);
        }
    }
    fn fmt_from(&self, f: &mut std::fmt::Formatter<'_>, node: NodeId) -> String {
        let n = self.arena[node].get();
        if let Some(x) = n {
            x.to_string()
        } else {
            let children = node.children(&self.arena).into_iter().collect::<Vec<_>>();
            assert_eq!(children.len(), 2);

            format!(
                "[{},{}]",
                //node,
                self.fmt_from(f, children[0]),
                self.fmt_from(f, children[1])
            )
        }
    }
    fn new_from_text(l: String) -> Self {
        let mut arena = Arena::new();

        let mut stack = Vec::new();
        let mut i = 0;
        let chars = l.chars().collect::<Vec<_>>();
        while i < chars.len() {
            match chars[i] {
                '[' => {}
                ',' => {}
                ']' => {
                    let id1 = stack.pop().unwrap();
                    let id2 = stack.pop().unwrap();
                    let parent = arena.new_node(None);
                    parent.append(id2, &mut arena);
                    parent.append(id1, &mut arena);

                    assert_eq!(parent.children(&arena).collect::<Vec<_>>().len(), 2);
                    stack.push(parent);
                }
                d if d.is_numeric() => {
                    let leaf = arena.new_node(Some(d.to_digit(10).unwrap() as usize));
                    stack.push(leaf);
                }
                o => panic!("invalid {}", o),
            }
            i += 1;
        }
        assert_eq!(stack.len(), 1);

        SnailfishNumber {
            arena,
            start: stack.into_iter().last().unwrap(),
        }
    }
}

fn get_input<P>(path: P) -> Vec<SnailfishNumber>
where
    P: AsRef<Path>,
{
    let content = fs::read_to_string(path).expect("war richtig");

    content
        .split("\n")
        .map(|l| SnailfishNumber::new_from_text(l.to_string()))
        .collect::<Vec<_>>()
}
fn main() {
    /*


    let nums2 = SnailfishNumber::new_from_text("[3,[1,[1,2]]]".to_string());

    nums.add(nums2);


    let mut nums =
        SnailfishNumber::new_from_text("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".to_string());
    println!("{}", &nums);

    let data = nums
        .leafs()
        .iter()
        .map(|x| nums.arena.get(*x).unwrap().get().unwrap())
        .collect::<Vec<_>>();
    //println!("{}", &nums.);
    println!("leaf {:?}", data);

    println!("deep {}", &nums);
    &nums.reduce();

    println!("deep {}", &nums);

     */
    let path = "src/input.txt";
    let mut nums = get_input(path);
    let (first, remaining) = nums.split_first_mut().unwrap();
    for n in remaining {
        first.add(n);
    }
    println!("Part1  {}", first.magnitude(first.start));

    let content = fs::read_to_string(path).expect("war richtig");

    let lines = content.split("\n").collect::<Vec<_>>();
    let mut m: u64 = 0;
    for x in 0..lines.len() {
        for y in 0..lines.len() {
            if x == y {
                continue;
            }
            let mut first = SnailfishNumber::new_from_text(lines[x].to_string());
            let second = SnailfishNumber::new_from_text(lines[y].to_string());
            first.add(&second);
            m = max(m, first.magnitude(first.start))
        }
    }

    println!("Part2  {}", m);
}

#[test]
fn mag_1() {
    let nums = SnailfishNumber::new_from_text(
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".to_string(),
    );

    assert_eq!(nums.magnitude(nums.start), 3488);
}

#[test]
fn mag_2() {
    let nums = SnailfishNumber::new_from_text("[[[[5,0],[7,4]],[5,5]],[6,6]]".to_string());

    assert_eq!(nums.magnitude(nums.start), 1137);
}
