use std::collections::HashSet;

use itertools::*;

const TEST_DATA: &str = include_str!("test.txt");
const INPUT_DATA: &str = include_str!("input.txt");
const MARGIN: i32 = 1;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Image {
    enhancement: Vec<bool>,
    image: HashSet<(i32, i32)>,
    max: (i32, i32),
    min: (i32, i32),
    default: i32,
}

impl Image {
    fn new(text: &str) -> Self {
        let (enhancement, image_str) = text.split_once("\n\n").unwrap();
        let enhancement = enhancement
            .chars()
            .filter(|&x| x != '\n')
            .map(|c| c == '#')
            .collect_vec();

        assert_eq!(enhancement.len(), 512);

        let mut image = HashSet::new();
        let mut row = 0;
        let mut col = 0;
        for line in image_str.split("\n") {
            col = 0;
            for c in line.chars() {
                if c == '#' {
                    image.insert((row, col));
                }
                col += 1;
            }
            row += 1;
        }

        assert_eq!(row, col);

        Self {
            enhancement,
            image,
            min: (0, 0),
            max: (row, col),
            default: 0,
        }
    }
    fn get_neighborhood(&self, row: i32, col: i32) -> i32 {
        let mut i: usize = 0;
        let undfault = (self.default ^ 1) as usize;
        for r in (row - 1)..=(row + 1) {
            for c in (col - 1)..=(col + 1) {
                i *= 2;
                i += match self.image.get(&(r, c)) {
                    Some(_) => undfault,
                    _ => self.default as usize,
                }
            }
        }
        if self.enhancement[i] {
            1
        } else {
            0
        }
    }

    fn enhance(&mut self) {
        let mut image = HashSet::new();
        let should_flip_default = self.enhancement[0];
        let new_default = if should_flip_default {
            self.default ^ 1
        } else {
            self.default
        };

        let (row_min, col_min) = self.min;
        let (row_max, col_max) = self.max;
        for row in (row_min - MARGIN)..=(row_max + MARGIN) {
            for col in (col_min - MARGIN)..=(col_max + MARGIN) {
                if new_default != self.get_neighborhood(row, col) {
                    image.insert((row, col));
                }
            }
        }
        self.image = image;
        self.min = (row_min - MARGIN, col_min - MARGIN);
        self.max = (row_max + MARGIN, col_max + MARGIN);
        self.default = new_default;
    }
    fn count_lit_pixels(&self) -> usize {
        if self.default == 0 {
            return self.image.len();
        }
        panic!("Lit is default");
    }

    fn count_lit_after_enhancement(&mut self, rounds: usize) -> usize {
        println!("Pixel {}", self.count_lit_pixels());
        for _ in 0..rounds {
            self.enhance();
        }
        self.count_lit_pixels()
    }
}

fn main() {
    let mut image = Image::new(INPUT_DATA);
    let lit = image.count_lit_after_enhancement(50);
    assert_eq!(lit, 17965);
    println!("Part 2 {}", lit);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_data_part2() {
        let mut image = Image::new(INPUT_DATA);
        assert_eq!(image.count_lit_after_enhancement(50), 17965)
    }

    #[test]
    fn input_data_part1() {
        let mut image = Image::new(INPUT_DATA);
        assert_eq!(image.count_lit_after_enhancement(2), 5571)
    }
    #[test]
    fn test_data_part1() {
        let mut image = Image::new(TEST_DATA);
        assert_eq!(image.count_lit_after_enhancement(2), 35)
    }
    #[test]
    fn test_data_part2() {
        let mut image = Image::new(TEST_DATA);
        assert_eq!(image.count_lit_after_enhancement(50), 3351)
    }
}
