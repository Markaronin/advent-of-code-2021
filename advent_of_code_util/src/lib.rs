use itertools::Itertools;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

pub fn read_lines_of_chars<P>(filename: P) -> Vec<Vec<char>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect_vec())
        .collect()
}

pub fn read_blocks<P>(filename: P) -> Vec<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    let mut blocks = vec![];
    let mut latest_block = vec![];
    for line in io::BufReader::new(file).lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            blocks.push(latest_block);
            latest_block = vec![];
        } else {
            latest_block.push(line);
        }
    }
    if !latest_block.is_empty() {
        blocks.push(latest_block);
    }
    blocks
}

pub fn split_block_on_whitespace(block: Vec<String>) -> Vec<String> {
    block
        .iter()
        .flat_map(|line| line.split_whitespace())
        .map(|split_line| split_line.to_string())
        .collect::<Vec<String>>()
}

pub fn abs_diff<T: Ord + std::ops::Sub<Output = T> + Copy>(slf: T, other: T) -> T {
    std::cmp::max(slf, other) - std::cmp::min(slf, other)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}
impl Coordinate {
    pub fn from_str(string: &str) -> Self {
        let (x, y) = string
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect_tuple::<(usize, usize)>()
            .unwrap();
        Coordinate { x, y }
    }

    /**
    assumes that from and to are either on a horizontal or vertical line
    */
    pub fn get_points_between_vertices(&self, to: &Coordinate) -> Vec<Coordinate> {
        assert!(self.x == to.x || self.y == to.y);
        if self.x == to.x {
            if self.y < to.y {
                return (self.y..=to.y)
                    .map(|y| Coordinate { x: self.x, y })
                    .collect();
            } else {
                return (to.y..=self.y)
                    .map(|y| Coordinate { x: self.x, y })
                    .collect();
            }
        } else {
            if self.x < to.x {
                return (self.x..=to.x)
                    .map(|x| Coordinate { x, y: self.y })
                    .collect();
            } else {
                return (to.x..=self.x)
                    .map(|x| Coordinate { x, y: self.y })
                    .collect();
            }
        }
    }

    pub fn is_within_bounds(&self, min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> bool {
        self.x >= min_x && self.x <= max_x && self.y >= min_y && self.y <= max_y
    }

    pub fn get_surrounding_non_diagonal_coordinates(
        &self,
        max_width: usize,
        max_height: usize,
    ) -> Vec<Coordinate> {
        let mut surrounding_coordinates = vec![];
        if self.x > 0 {
            surrounding_coordinates.push(Coordinate {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            surrounding_coordinates.push(Coordinate {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.x < max_width - 1 {
            surrounding_coordinates.push(Coordinate {
                x: self.x + 1,
                y: self.y,
            });
        }
        if self.y < max_height - 1 {
            surrounding_coordinates.push(Coordinate {
                x: self.x,
                y: self.y + 1,
            });
        }
        surrounding_coordinates
    }
    pub fn get_surrounding_coordinates(
        &self,
        max_width: usize,
        max_height: usize,
    ) -> Vec<Coordinate> {
        let mut surrounding_coordinates = vec![];
        if self.x > 0 {
            surrounding_coordinates.push(Coordinate {
                x: self.x - 1,
                y: self.y,
            });
            if self.y > 0 {
                surrounding_coordinates.push(Coordinate {
                    x: self.x - 1,
                    y: self.y - 1,
                });
            }
        }
        if self.y > 0 {
            surrounding_coordinates.push(Coordinate {
                x: self.x,
                y: self.y - 1,
            });
            if self.x < max_width - 1 {
                surrounding_coordinates.push(Coordinate {
                    x: self.x + 1,
                    y: self.y - 1,
                });
            }
        }
        if self.x < max_width - 1 {
            surrounding_coordinates.push(Coordinate {
                x: self.x + 1,
                y: self.y,
            });
            if self.y < max_height - 1 {
                surrounding_coordinates.push(Coordinate {
                    x: self.x + 1,
                    y: self.y + 1,
                });
            }
        }
        if self.y < max_height - 1 {
            surrounding_coordinates.push(Coordinate {
                x: self.x,
                y: self.y + 1,
            });
            if self.x > 0 {
                surrounding_coordinates.push(Coordinate {
                    x: self.x - 1,
                    y: self.y + 1,
                });
            }
        }
        surrounding_coordinates
    }
}

pub fn remove_first_and_last(string: &str) -> String {
    let mut chars = string.chars();
    chars.next();
    chars.next_back();
    chars.collect()
}

pub fn intersect_vectors<T: std::cmp::Ord>(vecs: Vec<Vec<T>>) -> Vec<T> {
    let mut vec_iter = vecs.into_iter();
    let mut remaining = BTreeSet::from_iter(vec_iter.next().unwrap());

    for vec in vec_iter {
        let vec_set = BTreeSet::from_iter(vec.into_iter());
        remaining = remaining
            .into_iter()
            .filter(|item| vec_set.contains(item))
            .collect();
    }

    remaining.into_iter().collect_vec()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn split_block_on_whitespace_test() {
        assert_eq!(
            split_block_on_whitespace(vec![
                "pid:161cm eyr:2025 hcl:#b6652a".to_string(),
                "cid:213".to_string(),
                "ecl:xry".to_string(),
                "hgt:150cm".to_string(),
                "iyr:2024 byr:2012".to_string()
            ]),
            vec![
                "pid:161cm".to_string(),
                "eyr:2025".to_string(),
                "hcl:#b6652a".to_string(),
                "cid:213".to_string(),
                "ecl:xry".to_string(),
                "hgt:150cm".to_string(),
                "iyr:2024".to_string(),
                "byr:2012".to_string()
            ]
        );
    }
}

#[macro_export]
macro_rules! base_aoc {
    ( $part_1_answer:literal, $part_2_answer:literal ) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn main() {
                let file_path = format!("{}/testinput", env!("CARGO_MANIFEST_DIR"));
                let (part_1_output, part_2_output) = get_program_output(&file_path);
                assert_eq!(part_1_output, $part_1_answer);
                assert_eq!(part_2_output, $part_2_answer);
            }
        }

        fn main() {
            let file_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
            let (part_1_output, part_2_output) = get_program_output(&file_path);
            println!("Part 1 output: {}", part_1_output);
            println!("Part 2 output: {}", part_2_output);
        }
    };
}
