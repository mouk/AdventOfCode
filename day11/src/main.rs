use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Octopuses {
    data: Vec<Vec<u32>>,
    columns: usize,
    rows: usize,
    flashes_count: usize,
    steps_count: usize,
}

impl Octopuses {
    fn new(content: String) -> Self {
        let data: Vec<Vec<u32>> = content
            .split("\r\n")
            .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
            .collect();

        println!("Data {:?}", data);
        let columns = data[0].len();
        let rows = data.len();
        return Self {
            data: data,
            columns: columns,
            rows: rows,
            flashes_count: 0,
            steps_count: 0
        };
    }

    fn get(&self, col: usize, row: usize) -> u32 {
        self.data[row][col]
    }

    fn run_step(&mut self) -> usize{
        let mut flashed = HashSet::new();
        for col in 0..self.columns {
            for row in 0..self.rows {
                self.data[row][col] += 1;
                if self.data[row][col] > 9 {
                    flashed.insert((row, col));
                }
            }
        }
        let mut queue = flashed.iter().map(|x| *x).collect::<Vec<(usize, usize)>>();
        loop {
            match queue.pop() {
                Some((r, c)) => {
                    for (row, col) in self.get_neighbors(r, c) {
                        if !flashed.contains(&(row, col)) {
                            self.data[row][col] += 1;
                            if self.data[row][col] > 9 {
                                flashed.insert((row, col));
                                queue.push((row, col));
                            }
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }
        let flash_count =  flashed.len();
        self.flashes_count +=flash_count;
        for (row, col) in flashed {
            self.data[row][col] = 0
        }
        self.steps_count += 1;
        flash_count
    }
    fn get_neighbors(&mut self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();
        for r in (row as i32 - 1)..=(row  as i32+ 1) {
            for c in (col  as i32- 1)..=(col as i32+ 1) {
                if self.is_valid(r, c) {
                    result.push((r as usize, c as usize));
                }
            }
        }
        result
    }

    fn is_valid(&mut self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.rows.try_into().unwrap() && col >= 0 && col < self.columns.try_into().unwrap()
    }

    fn print(&self) {
        println!("----{}----", self.flashes_count);
        for line in self.data.iter().map(|row| {
            row.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
        }) {
            println!("{}", line);
        }
        
        println!("{}", "--------------------");
    }
}
fn main() {
    let content = fs::read_to_string("src/input.txt").expect("war richtig");
    println!("{:?}", &content);
    let mut octos = Octopuses::new(content);
    for _ in 0..100{
        octos.run_step();
    }
    
    octos.print();

    println!("Part 1 {}", octos.flashes_count);
    loop {
        let flashes = octos.run_step();
        if flashes ==  octos.columns * octos.rows{
            println!("Part 2 {}", octos.steps_count);
            break;
        }

    }

}
