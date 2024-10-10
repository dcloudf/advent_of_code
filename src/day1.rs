use std::cmp::{max_by_key, min_by_key};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./data/day1.txt") {
        println!(
            "{}",
            lines
                .into_iter()
                .map(|x| process_line(x.unwrap().as_str()))
                .sum::<i32>()
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

fn process_line(line: &str) -> i32 {
    let digits_structs = look_for_digits(line);
    if let Some(digit_words_structs) = look_for_digit_words(line) {
        min_by_key(
            digits_structs.0,
            digit_words_structs.0,
            |x: &DigitFindingResult| x.position,
        )
        .value
            * 10
            + max_by_key(
                digits_structs.1,
                digit_words_structs.1,
                |x: &DigitFindingResult| x.position,
            )
            .value
    } else {
        digits_structs.0.value * 10 + digits_structs.1.value
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct DigitFindingResult {
    value: i32,
    position: usize,
}

fn look_for_digits(line: &str) -> (DigitFindingResult, DigitFindingResult) {
    let first_position = line.find(|x: char| x.is_ascii_digit()).unwrap();
    let last_position = line.rfind(|x: char| x.is_ascii_digit()).unwrap();
    (
        DigitFindingResult {
            value: line
                .chars()
                .nth(first_position)
                .unwrap()
                .to_digit(10)
                .unwrap() as i32,
            position: first_position,
        },
        DigitFindingResult {
            value: line
                .chars()
                .nth(last_position)
                .unwrap()
                .to_digit(10)
                .unwrap() as i32,
            position: last_position,
        },
    )
}

fn look_for_digit_words(line: &str) -> Option<(DigitFindingResult, DigitFindingResult)> {
    let words = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut results: Vec<DigitFindingResult> = Vec::new();
    for (key, value) in words.iter() {
        if let Some(position) = line.find(key) {
            results.push(DigitFindingResult {
                value: *value,
                position,
            });
        }
        if let Some(position) = line.rfind(key) {
            results.push(DigitFindingResult {
                value: *value,
                position,
            });
        }
    }
    match results.is_empty() {
        true => Option::None,
        false => Option::Some((
            *results.iter().min_by_key(|x| x.position).unwrap(),
            *results.iter().max_by_key(|x| x.position).unwrap(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_line() {
        assert_eq!(process_line("1abc2"), 12);
        assert_eq!(process_line("pqr3stu8vwx"), 38);
        assert_eq!(process_line("a1b2c3d4e5f"), 15);
        assert_eq!(process_line("4nineeightseven2"), 42);
        assert_eq!(process_line("zoneight234"), 14);
        assert_eq!(process_line("7pqrstsixteen"), 76);
    }

    #[test]
    fn test_look_for_digit_words() {
        assert_eq!(
            look_for_digit_words("two1nine"),
            Some((
                DigitFindingResult {
                    value: 2,
                    position: 0
                },
                DigitFindingResult {
                    value: 9,
                    position: 4
                }
            ))
        );
        assert_eq!(
            look_for_digit_words("eightwothree"),
            Some((
                DigitFindingResult {
                    value: 8,
                    position: 0
                },
                DigitFindingResult {
                    value: 3,
                    position: 7
                }
            ))
        );
        assert_eq!(
            look_for_digit_words("abcone2threexyz"),
            Some((
                DigitFindingResult {
                    value: 1,
                    position: 3
                },
                DigitFindingResult {
                    value: 3,
                    position: 7
                }
            ))
        );
        assert_eq!(
            look_for_digit_words("xtwone3four"),
            Some((
                DigitFindingResult {
                    value: 2,
                    position: 1
                },
                DigitFindingResult {
                    value: 4,
                    position: 7
                }
            ))
        );
        assert_eq!(
            look_for_digit_words("4nineeightseven2"),
            Some((
                DigitFindingResult {
                    value: 9,
                    position: 1
                },
                DigitFindingResult {
                    value: 7,
                    position: 10
                }
            ))
        );
        assert_eq!(
            look_for_digit_words("zoneight234"),
            Some((
                DigitFindingResult {
                    value: 1,
                    position: 1
                },
                DigitFindingResult {
                    value: 8,
                    position: 3
                }
            ))
        );
        assert_eq!(
            look_for_digit_words("7pqrstsixteen"),
            Some((
                DigitFindingResult {
                    value: 6,
                    position: 6
                },
                DigitFindingResult {
                    value: 6,
                    position: 6
                }
            ))
        );
    }

    #[test]
    fn test_look_for_digits() {
        assert_eq!(
            look_for_digits("1abc2"),
            (
                DigitFindingResult {
                    value: 1,
                    position: 0
                },
                DigitFindingResult {
                    value: 2,
                    position: 4
                }
            )
        );
        assert_eq!(
            look_for_digits("pqr3stu8vwx"),
            (
                DigitFindingResult {
                    value: 3,
                    position: 3
                },
                DigitFindingResult {
                    value: 8,
                    position: 7
                }
            )
        );
        assert_eq!(
            look_for_digits("a1b2c3d4e5f"),
            (
                DigitFindingResult {
                    value: 1,
                    position: 1
                },
                DigitFindingResult {
                    value: 5,
                    position: 9
                }
            )
        );
        assert_eq!(
            look_for_digits("treb7uchet"),
            (
                DigitFindingResult {
                    value: 7,
                    position: 4
                },
                DigitFindingResult {
                    value: 7,
                    position: 4
                }
            )
        );
        assert_eq!(
            look_for_digits("4nineeightseven2"),
            (
                DigitFindingResult {
                    value: 4,
                    position: 0
                },
                DigitFindingResult {
                    value: 2,
                    position: 15
                }
            )
        );
    }
}
