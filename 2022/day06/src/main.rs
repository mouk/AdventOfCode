struct MarkDetector<const N: usize>{
    buffer: [char;N],
    pointer: usize
}
impl<const N: usize> MarkDetector<N>{
    fn new() ->Self{
        Self{
            buffer: [' '; N ],
            pointer: 0
        }
    }
    fn add(&mut self, c: char){
       self.buffer[self.pointer] = c;
       self.pointer = (self.pointer + 1) % N;
    }

    fn is_marker(&self) -> bool{
        for index in 0..N{
            if self.buffer[index] == ' ' {
                return false;
            }
            for y in (index+1)..N{
                if self.buffer[index] == self.buffer[y] {
                    return false;
                }
            }
            
        }
        true
     }
     fn find_marker(input: &str)-> usize{
        let mut char_read = 0;
        let mut detector = MarkDetector::<N>::new();
        for c in input.chars(){
            char_read += 1;
            detector.add(c);
            if detector.is_marker(){
                return char_read;
            }
        }
        char_read

     }
}
fn main() {

    const INPUT_DATA: &str = include_str!("input.txt");
    println!("Part1: {}", MarkDetector::<4>::find_marker(INPUT_DATA));
    println!("Part2: {}", MarkDetector::<14>::find_marker(INPUT_DATA));
}
