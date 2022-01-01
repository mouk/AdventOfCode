#[derive(Copy, Clone, Debug)]
struct World {
    players: [usize; 4],
    current_player: usize,
    possible_paths: u64,
}

impl World {
    fn new(space_1: usize, space_2: usize) -> Self {
        World {
            players: [space_1, 0, space_2, 0],
            current_player: 0,
            possible_paths: 1,
        }
    }

    fn winner(&self) -> i8 {
        if self.players[1] >= 21 {
            //println!("Player 1 won {:?}", self.players);
            return 0;
        } else if self.players[3] >= 21 {
            //println!("Player 2 won {:?}", self.players);
            return 1;
        }

        return -1;
    }
    fn move_steps(&mut self, steps: usize, possibilities: u64) -> i8 {
        let pos = 2 * self.current_player;
        self.players[pos] = ((self.players[pos] + (steps - 1)) % 10) + 1;
        self.players[pos + 1] += self.players[pos];
        self.current_player = (self.current_player + 1) % 2;
        self.possible_paths *= possibilities;
        self.winner()
    }
}
fn main() {
    let mut possibilities = [0; 7];
    let mut winning_paths: [u64; 2] = [0, 0];
    for i in 0..3 {
        for ii in 0..3 {
            for iii in 0..3 {
                possibilities[i + ii + iii] += 1;
            }
        }
    }
    println!("possibilities {:?}.", possibilities);

    //let mut worlds = vec![World::new(4, 8)];
    let mut worlds = vec![World::new(4, 10)];

    while let Some(current) = worlds.pop() {
        //println!("next {:?}", current);
        for i in 0..7 {
            let mut next = current.clone();
            match next.move_steps(i + 3, possibilities[i]) {
                0 => winning_paths[0] += next.possible_paths,
                1 => winning_paths[1] += next.possible_paths,
                _ => worlds.push(next),
            }
            // println!("New {:?}.", next);
        }
        //println!("New queu {:?}", worlds.len());
    }

    println!("All {:?}.", winning_paths);
    println!("Part 2 {:?}.", winning_paths.iter().max().unwrap());
}
