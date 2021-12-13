use std::fs;
#[derive(Debug)]
enum ChunkState{
    Valid,
    Corrupted(char),
    Incomplete(Vec<char>)
}

fn main() {

    let content = fs::read_to_string("src/input.txt")
    .expect("war richtig");

    let lines = content
    .split("\r\n")
    .map(|line| verify_chunk(line))
    .collect::<Vec<ChunkState>>();


    let part1:i32 = lines.iter()
    .map(|state| match *state{
        ChunkState::Corrupted(')') => 3,
        ChunkState::Corrupted(']') => 57,
        ChunkState::Corrupted('}') => 1197,
        ChunkState::Corrupted('>') => 25137,
        _ => 0,
    })
    .sum();
    
    println!("Part 1: {:?}", part1);

    
    let mut part2:Vec<u64> = lines
    .iter()
    .map(|state| match state{
        ChunkState::Incomplete(stack) =>{
            let mapped = stack.iter().map(|x| match x{
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _=> 0
            }).rev();
            let mut score:u64 = 0;
            for n in mapped{
                score *=5;
                score += n;
            }
            
            println!("incomplete {:?}, scoe {}", stack, score);
            return score;
        },
        _ => 0,
    })
    .filter(|x| *x > 0)
    .collect();
    part2.sort();

    let middle = usize::from((part2.len() - 1)/2);
    
    println!("Scores {:?}", part2);
    println!("Part 2: {:?}", part2[middle]);
}
fn verify_chunk(chunk: &str)-> ChunkState{
    
    let mut stack:Vec<char> = Vec::new();

    for c in chunk.chars(){
        if c == '(' ||  c == '[' || c == '{' || c == '<'{
            stack.push(c)
        }else {
            let matching = stack.pop();
            match (matching, c){
                (Some('('), ')') |
                (Some('{'), '}') |
                (Some('['), ']') |
                (Some('<'), '>') =>  {},
                _ => return ChunkState::Corrupted(c)
            }
        }
    }
    if stack.len() == 0{
        return ChunkState::Valid;
    }else{
        return ChunkState::Incomplete(stack);
    }
    
}
