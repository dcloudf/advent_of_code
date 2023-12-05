use std::fs::read_to_string;

fn main() {
    let (numbers, symbols) = parse_engine_schematic("./data/day3.txt");
    println!("{}", filter_numbers(&numbers, &symbols).iter().sum::<u32>());
    println!("{}", calc_gear_ratios(symbols, numbers).iter().sum::<u32>());
}

#[derive(Debug, PartialEq)]
struct Symbol {
    position: (i32, i32),
    value: char,
}

#[derive(Debug, PartialEq)]
struct Number {
    value: u32,
    row: i32,
    start: i32,
    end: i32,
}

fn parse_engine_schematic(filename: &str) -> (Vec<Number>, Vec<Symbol>) {
    let file_text = read_to_string(filename).unwrap();
    let enum_lines = file_text.lines().enumerate();
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    for enum_line in enum_lines {
        let mut enum_chars = enum_line.1.chars().enumerate();
        while let Some(char) = enum_chars.next() {
            if char.1.is_ascii_digit() {
                let mut digits = String::from(char.1);
                let start = char.0;
                let mut end = char.0;
                for ch in enum_chars.by_ref() {
                    if ch.1 == '.' {
                        break;
                    } else if !ch.1.is_ascii_digit() {
                        symbols.push(Symbol {
                            position: (ch.0 as i32, enum_line.0 as i32),
                            value: ch.1,
                        });
                        break;
                    }
                    digits.push(ch.1);
                    end += 1;
                }
                numbers.push(Number {
                    value: digits.parse::<u32>().unwrap(),
                    row: enum_line.0 as i32,
                    start: start as i32,
                    end: end as i32,
                });
            } else if char.1 != '.' {
                symbols.push(Symbol {
                    position: (char.0 as i32, enum_line.0 as i32),
                    value: char.1,
                })
            }
        }
    }
    (numbers, symbols)
}

fn filter_numbers(numbers: &Vec<Number>, symbols: &Vec<Symbol>) -> Vec<u32> {
    numbers
        .into_iter()
        .filter(|num| {
            symbols.iter().any(|s| {
                (num.start - 1 <= s.position.0 && s.position.0 <= num.end + 1)
                    && (s.position.1 == num.row
                        || s.position.1 == num.row - 1
                        || s.position.1 == num.row + 1)
            })
        })
        .map(|x| x.value)
        .collect()
}

fn calc_gear_ratios(symbols: Vec<Symbol>, numbers: Vec<Number>) -> Vec<u32> {
    symbols
        .iter()
        .filter(|x| x.value == '*')
        .filter(|s| {
            numbers
                .iter()
                .filter(|num| {
                    (num.start - 1 <= s.position.0 && s.position.0 <= num.end + 1)
                        && (s.position.1 == num.row
                            || s.position.1 == num.row - 1
                            || s.position.1 == num.row + 1)
                })
                .count()
                == 2
        })
        .map(|s| {
            numbers
                .iter()
                .filter(|num| {
                    (num.start - 1 <= s.position.0 && s.position.0 <= num.end + 1)
                        && (s.position.1 == num.row
                            || s.position.1 == num.row - 1
                            || s.position.1 == num.row + 1)
                })
                .map(|x| x.value)
                .reduce(|x, y| x * y)
                .unwrap()
        })
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_part1() {
        let (numbers, symbols) = parse_engine_schematic("./data/test_day3.txt");
        assert_eq!(filter_numbers(&numbers, &symbols).iter().sum::<u32>(), 4361)
    }

    #[test]
    fn test_day3_part2() {
        let (numbers, symbols) = parse_engine_schematic("./data/test_day3.txt");
        assert_eq!(
            calc_gear_ratios(symbols, numbers).iter().sum::<u32>(),
            467835
        )
    }

    #[test]
    fn test_filter_numbers() {
        assert_eq!(
            filter_numbers(
                &Vec::from([
                    Number {
                        value: 467,
                        row: 0,
                        start: 0,
                        end: 2
                    },
                    Number {
                        value: 114,
                        row: 0,
                        start: 5,
                        end: 7
                    }
                ]),
                &Vec::from([Symbol {
                    position: (3, 1),
                    value: '*'
                }])
            ),
            Vec::from([467,])
        )
    }
}
