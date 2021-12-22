use std::cmp::max;

#[derive(Debug)]
struct TargetArea {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}
#[derive(Debug)]
struct Projectile {
    position: (i32, i32),
    velocity: (i32, i32),
    initial_velocity: (i32, i32),
    highest_y: i32,
}

impl Projectile {
    fn new_from_coordinates(x: i32, y: i32) -> Self {
        Self {
            position: (0, 0),
            velocity: (x, y),
            initial_velocity: (x, y),
            highest_y: 0,
        }
    }
    fn move_step(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        let x_velocity = if self.velocity.0 == 0 {
            0
        } else if self.velocity.0 < 0 {
            1
        } else {
            -1
        };
        self.velocity.0 += x_velocity;
        self.velocity.1 -= 1;

        self.highest_y = max(self.highest_y, self.position.1)
    }
}

impl TargetArea {
    fn contains(&self, p: &Projectile) -> bool {
        let x = p.position.0;
        let y = p.position.1;
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }

    fn missed_by(&self, p: &Projectile) -> bool {
        p.position.0 > self.max_x || p.position.1 < self.min_y
    }

    fn run_projectile_through(&self, mut p: Projectile) -> Option<Projectile> {
        loop {
            p.move_step();
            if self.missed_by(&p) {
                return None;
            }

            if self.contains(&p) {
                return Some(p);
            }
        }
    }
    fn find_all(&self) -> Vec<Projectile> {
        let mut result = Vec::new();
        let max_x = self.max_x;
        let max_y = max(self.max_y.abs(), self.min_y.abs());
        println!("max x {}, max y {}", max_x, &max_y);

        for x in 2..max_x {
            for y in (-max_y)..max_y {
                let p = Projectile::new_from_coordinates(x, y);
                if let Some(p) = self.run_projectile_through(p) {
                    //println!("New Point {:?}", p);
                    result.push(p);
                }
            }
        }
        result
    }

    fn get_highest_trajectory(&self) -> Projectile {
        let all = self.find_all();
        all.into_iter().max_by_key(|p| p.highest_y).unwrap()
    }
}
fn main() {
    let area = TargetArea {
        min_x: 192,
        max_x: 251,
        min_y: -89,
        max_y: -59,
    };

    let highest = area.get_highest_trajectory();

    println!("result  {:?}", highest);
}

#[test]
fn test_part1() {
    let area = TargetArea {
        min_x: 20,
        max_x: 30,
        min_y: -10,
        max_y: -5,
    };

    let highest = area.get_highest_trajectory();
    assert_eq!(highest.highest_y, 45)
}

#[test]
fn input_part1() {
    let area = TargetArea {
        min_x: 192,
        max_x: 251,
        min_y: -89,
        max_y: -59,
    };

    let highest = area.get_highest_trajectory();
    assert_eq!(highest.highest_y, 3916)
}
