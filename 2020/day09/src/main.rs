
const TEST_DATA: &str = include_str!("test.txt");
const INPUT_DATA: &str = include_str!("input.txt");
struct Xmas{
    preamble: usize,
    numbers: Vec<u64>
}
impl Xmas {
    fn new(input: &str, preamble: usize) -> Self{
        let numbers = input.split("\n")
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
        Self{
            numbers,
            preamble
        }
    }
    fn get_invalid(&self) -> Option<u64>{

        'candidate_loop: for x in self.preamble..self.numbers.len(){
            let candidate = self.numbers[x];
            for start in (x- self.preamble)..(x-1){
                for end in (start+1)..x{
                    if self.numbers[start]  + self.numbers[end] ==  candidate{
                        continue 'candidate_loop;
                    }
                }
            }
            return Some(candidate)
        }
        None
    }
    fn find_weakness(&self) -> Option<u64>{
        let invalid = self.get_invalid()?;
        for start in 0..self.numbers.len(){
            let mut sum = self.numbers[start];
            let mut end = start;
            while sum < invalid && end < (self.numbers.len() -1){
                end += 1;
                sum += self.numbers[end];
                if sum == invalid{
                    let mut min = self.numbers[start];
                    let mut max = self.numbers[start];
                    for x in start..=end{
                        if self.numbers[x] > max{
                            max = self.numbers[x];
                        }
                        if self.numbers[x] < min{
                            min = self.numbers[x];
                        }
                    }

                    println!("min: {:?}, max: {:?}", min,max);
                   return Some(min+max);
                }
            }
        }

        None

    }
    
}
fn main() {
    let xmas = Xmas::new(INPUT_DATA, 25);
    let result = xmas.get_invalid();
    println!("Part 1: {:?}", result.unwrap());
    let find_weakness = xmas.find_weakness();
    println!("Part 2: {:?}", find_weakness.unwrap());


}

#[test]
fn part1_test_data() {
    let xmas = Xmas::new(TEST_DATA, 5);
    let result = xmas.get_invalid();

    assert_eq!(result, Some(127));
}

#[test]
fn part2_test_data() {
    let xmas = Xmas::new(TEST_DATA, 5);
    let result = xmas.find_weakness();

    assert_eq!(result, Some(62));
}

#[test]
fn part1_input_data() {
    let xmas = Xmas::new(INPUT_DATA, 25);
    let result = xmas.get_invalid();

    assert_eq!(result, Some(542529149));
}

