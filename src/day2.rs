use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Add;
use std::path::Path;

fn main() {
    let reference = Subset {
        red: 12,
        green: 13,
        blue: 14,
    };
    if let Ok(lines) = read_lines("./data/day2.txt") {
        println!(
            "{}",
            lines
                .into_iter()
                .map(|line| parse_game_info(line.unwrap().as_str()))
                .filter(|result| result
                    .subsets
                    .clone()
                    .iter()
                    .all(|x| reference.is_possible_to_take(x)))
                .map(|x| x.id)
                .sum::<u32>()
        );
    }
    if let Ok(lines) = read_lines("./data/day2.txt") {
        println!(
            "{}",
            lines
                .into_iter()
                .map(|line| parse_game_info(line.unwrap().as_str()))
                .map(|x| x.sets_power())
                .sum::<u32>()
        );
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialOrd, PartialEq)]
struct GameResult {
    id: u32,
    subsets: Vec<Subset>,
}

impl GameResult {
    fn sets_power(&self) -> u32 {
        self.subsets.iter().map(|x| x.red).max().unwrap()
            * self.subsets.iter().map(|x| x.green).max().unwrap()
            * self.subsets.iter().map(|x| x.blue).max().unwrap()
    }
}

#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
struct Subset {
    red: u32,
    green: u32,
    blue: u32,
}

impl Subset {
    fn new(raw_string: &str) -> Self {
        let mut temp_table = HashMap::new();
        for subpart in raw_string.split(',') {
            let mut splitted = subpart.split_whitespace();
            let (amount, color) = (
                splitted.next().unwrap().parse::<u32>().unwrap(),
                splitted.next().unwrap(),
            );
            temp_table.insert(color, amount);
        }
        Self {
            red: *temp_table.get("red").unwrap_or(&0),
            green: *temp_table.get("green").unwrap_or(&0),
            blue: *temp_table.get("blue").unwrap_or(&0),
        }
    }

    fn is_possible_to_take(self, other: &Subset) -> bool {
        other.red <= self.red && other.green <= self.green && other.blue <= self.blue
    }
}

impl Add for Subset {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

fn parse_game_info(line: &str) -> GameResult {
    let mut splitted = line.split(':');
    let (game_part, cubes_part) = (splitted.next().unwrap(), splitted.next().unwrap());
    let game_id = game_part
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let subsets = cubes_part.split(';').map(Subset::new).collect();
    GameResult {
        id: game_id,
        subsets,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game_info() {
        assert_eq!(
            parse_game_info("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            GameResult {
                id: 1,
                subsets: Vec::from([
                    Subset {
                        blue: 3,
                        red: 4,
                        green: 0
                    },
                    Subset {
                        blue: 6,
                        green: 2,
                        red: 1
                    },
                    Subset {
                        green: 2,
                        blue: 0,
                        red: 0
                    }
                ])
            }
        );
        assert_eq!(
            parse_game_info("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            GameResult {
                id: 2,
                subsets: Vec::from([
                    Subset {
                        blue: 1,
                        green: 2,
                        red: 0
                    },
                    Subset {
                        green: 3,
                        blue: 4,
                        red: 1
                    },
                    Subset {
                        green: 1,
                        blue: 1,
                        red: 0
                    }
                ])
            }
        );
        assert_eq!(
            parse_game_info(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
            ),
            GameResult {
                id: 3,
                subsets: Vec::from([
                    Subset {
                        red: 20,
                        green: 8,
                        blue: 6
                    },
                    Subset {
                        green: 13,
                        blue: 5,
                        red: 4
                    },
                    Subset {
                        red: 1,
                        green: 5,
                        blue: 0
                    }
                ])
            }
        );
    }

    #[test]
    fn test_subset_comparison() {
        let reference = Subset {
            red: 20,
            green: 8,
            blue: 6,
        };
        let overflow = Subset {
            red: 25,
            green: 26,
            blue: 11,
        };
        assert_eq!(reference.is_possible_to_take(&overflow), false);
    }

    #[test]
    fn test_subsets_power() {
        assert_eq!(
            GameResult {
                id: 1,
                subsets: Vec::from([
                    Subset {
                        blue: 3,
                        red: 4,
                        green: 0
                    },
                    Subset {
                        blue: 6,
                        green: 2,
                        red: 1
                    },
                    Subset {
                        green: 2,
                        blue: 0,
                        red: 0
                    }
                ])
            }
            .sets_power(),
            48
        );
        assert_eq!(
            GameResult {
                id: 2,
                subsets: Vec::from([
                    Subset {
                        blue: 1,
                        green: 2,
                        red: 0
                    },
                    Subset {
                        green: 3,
                        blue: 4,
                        red: 1
                    },
                    Subset {
                        green: 1,
                        blue: 1,
                        red: 0
                    }
                ])
            }
            .sets_power(),
            12
        );
        assert_eq!(
            GameResult {
                id: 3,
                subsets: Vec::from([
                    Subset {
                        red: 20,
                        green: 8,
                        blue: 6
                    },
                    Subset {
                        green: 13,
                        blue: 5,
                        red: 4
                    },
                    Subset {
                        red: 1,
                        green: 5,
                        blue: 0
                    }
                ])
            }
            .sets_power(),
            1560
        );
    }
}
