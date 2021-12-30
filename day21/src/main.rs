use itertools::*;

struct Player {
    score: usize,
    current_space: usize,
}
struct Dice {
    current: usize,
    total_rolled: usize,
}
struct World {
    players: [Player; 2],
    dice: Dice,
}

impl World {
    fn new(space_1: usize, space_2: usize) -> Self {
        World {
            players: [
                Player {
                    score: 0,
                    current_space: space_1,
                },
                Player {
                    score: 0,
                    current_space: space_2,
                },
            ],
            dice: Dice {
                current: 0,
                total_rolled: 0,
            },
        }
    }

    fn play_to_end(&mut self) -> usize {
        let mut current_player = 0;
        while !self.has_winner() {
            let add = self.dice.next() + self.dice.next() + self.dice.next();
            self.players[current_player].move_steps(add);

            /*

            println!(
                "Player {} rolls {} and moves to space {} for a total score of {}.",
                current_player + 1,
                add,
                self.players[current_player].current_space,
                self.players[current_player].score
            );
             */
            current_player = (current_player + 1) % 2;
        }
        self.loosing_score()
    }

    fn has_winner(&self) -> bool {
        self.players.iter().any(Player::is_winner)
    }

    fn loosing_score(&self) -> usize {
        if !self.has_winner() {
            panic!("Not finished yet")
        }

        let looser = self.players.iter().find(|&x| !x.is_winner()).unwrap();
        looser.score * self.dice.total_rolled
    }
}

impl Player {
    fn is_winner(&self) -> bool {
        self.score >= 1000
    }
    fn move_steps(&mut self, steps: usize) {
        self.current_space = (((self.current_space - 1) + steps) % 10) + 1;
        self.score += self.current_space;
    }
}
impl Dice {
    fn next(&mut self) -> usize {
        let mut next = self.current + 1;
        if next > 100 {
            next -= 100;
        }
        self.current = next;
        self.total_rolled += 1;
        next
    }
}
fn main() {
    let mut world = World::new(4, 10);
    let total_score = world.play_to_end();

    println!("Totsl score {}", total_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_data_part2() {
        let mut world = World::new(4, 8);
        let total_score = world.play_to_end();

        assert_eq!(total_score, 739785)
    }
}
