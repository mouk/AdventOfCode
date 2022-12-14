use std::collections::HashSet;

use itertools::Itertools;


fn solve1(input: &str) -> usize {
    input
        .split('\n')
        .map(|l| {
            let mid = l.len() / 2;
            let (f, s) = l.split_at(mid);
            (
                f.chars().collect::<HashSet<_>>(),
                s.chars().collect::<HashSet<_>>(),
            )
        })
        .map(|(f, s)| f.intersection(&s).map(get_priority).sum::<usize>())
        .sum()
}

fn solve2(input: &str) -> usize {
    
    input
        .split('\n')
        .map(|l| l.chars().collect::<HashSet<_>>())
        .tuples::<(_,_,_)>()
        .map(|(a,b,c)|  {
        let intersection = a.intersection(&b).map(|ch| *ch).collect::<HashSet<char>>();
        let badge = intersection.intersection(&c).map(|ch| *ch).collect::<Vec<_>>()[0];
        get_priority(&badge)
})
.sum()
}

fn get_priority(c: &char) -> usize {
    let val = *c as usize;
    if val > 96 {
        val - 96
    } else {
        val - 38
    }
}

fn main() {
    
    const INPUT_DATA: &str = include_str!("INPUT.txt");
    println!("Part1 {}", solve1(INPUT_DATA));


    println!("Part2 {}", solve2(INPUT_DATA));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        const TEST_DATA: &str = include_str!("test.txt");
        let sol = solve1(TEST_DATA);
        assert_eq!(sol, 157)
    }

    #[test]
    fn test0() {
        assert_eq!(get_priority(&'a'), 1);
        assert_eq!(get_priority(&'z'), 26);
        assert_eq!(get_priority(&'A'), 27);
    }
}
