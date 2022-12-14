
const INPUT: &str = include_str!("input1.txt");

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
enum Hand {
    Rock = 0,
    Paper = 1,
    Scissors = 2
}
impl Hand{
    fn to_value(&self) -> i32{
        (*self as i32) + 1
    }

    fn to_winning_points(&self, other: &Self) -> i32{
        if other == self{
            return 3
        }
        match (self, other){
            (Hand::Rock, Hand::Scissors) | (Hand::Paper, Hand::Rock) | (Hand::Scissors, Hand::Paper)=> 6,
            _ => 0
        }
    }
    fn from_elf(l: &str) -> Self{
        if l == "A" {
            return Hand::Rock
        }if l == "B" {
            return Hand::Paper
        }if l == "C" {
            return Hand::Scissors
        }
        panic!("{} not supported", l)
    }

    fn from_own(l: &str) -> Self{
        if l == "X" {
            return Hand::Rock
        }if l == "Y" {
            return Hand::Paper
        }if l == "Z" {
            return Hand::Scissors
        }
        panic!("{} not supported", l)
    }

    fn from_strategy(&self, strategy: &str) -> Self{
        let offset = match strategy{
            "X"  => -1,
            "Y"  => 0,
            _ => 1
        };

        let result  = ((*self as i32) + offset) % 3;
        match result{
            0 => Hand::Rock,
            1 => Hand::Paper,
            _ => Hand::Scissors
        }
    }
}


fn solve2(input: &str)-> i32{
    let hands= input
    .split('\n')
    .map(|l| {
        let (f,s)  = l.split_once(' ').unwrap();
        let elf = Hand::from_elf(f);
        let me = elf.from_strategy(s);
        (elf,me)
    });
    solve_hands(hands)
}

fn solve1(input: &str)-> i32{
    let hands = input
    .split('\n')
    .map(|l| {
        let (f,s)  = l.split_once(' ').unwrap();
        (Hand::from_elf(f), Hand::from_own(s))
    });
    solve_hands(hands)
}

fn solve_hands(input: impl Iterator<Item=(Hand,Hand)>)-> i32{
    input
   .map(|(elf, me)| {
        me.to_value() + me.to_winning_points(&elf)
    })
    .sum()
}

fn main() {
    let result:i32 = solve1(INPUT);

    println!("Result1: {:?}", result);


    let result:i32 = solve2(INPUT);

    println!("Result2: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA: &str = include_str!("test.txt");
    #[test]
    fn test1() {
        let result = solve1(TEST_DATA);
        assert_eq!(result, 15)
    }
}
