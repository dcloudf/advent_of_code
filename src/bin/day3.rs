use std::fs::read_to_string;

fn main() {
    let (numbers, symbols) = parse_engine_schematic("./data/day3.txt");
    println!("{}", filter_numbers(numbers, symbols).iter().sum::<u32>())
}

#[derive(Debug, PartialEq)]
struct Point(i32, i32);

#[derive(Debug, PartialEq)]
struct Number {
    value: u32,
    row: i32,
    start: i32,
    end: i32,
}

fn parse_engine_schematic(filename: &str) -> (Vec<Number>, Vec<Point>) {
    let file_text = read_to_string(filename).unwrap();
    let mut enum_lines = file_text.lines().enumerate();
    let mut numbers = Vec::new();
    let mut symbol_positions = Vec::new();
    while let Some(enum_line) = enum_lines.next() {
        let mut enum_chars = enum_line.1.chars().enumerate();
        while let Some(char) = enum_chars.next() {
            if char.1.is_ascii_digit() {
                let mut digits = String::from(char.1);
                let start = char.0;
                let mut end = char.0;
                while let Some(ch) = enum_chars.next() {
                    if ch.1 == '.' {
                        break;
                    } else if !ch.1.is_ascii_digit() {
                        symbol_positions.push(Point(ch.0 as i32, enum_line.0 as i32));
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
                symbol_positions.push(Point(char.0 as i32, enum_line.0 as i32))
            }
        }
    }
    (numbers, symbol_positions)
}

fn filter_numbers(numbers: Vec<Number>, symbols: Vec<Point>) -> Vec<u32> {
    numbers
        .into_iter()
        .filter(|num| {
            symbols.iter().any(|s| {
                (num.start - 1 <= s.0 && s.0 <= num.end + 1)
                    && (s.1 == num.row || s.1 == num.row - 1 || s.1 == num.row + 1)
            })
        })
        .map(|x| x.value)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3() {
        let (numbers, symbols) = parse_engine_schematic("./data/test_day3.txt");
        assert_eq!(filter_numbers(numbers, symbols).iter().sum::<u32>(), 4361)
    }

    #[test]
    fn test_filter_numbers() {
        assert_eq!(
            filter_numbers(
                Vec::from([
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
                Vec::from([Point(3, 1)])
            ),
            Vec::from([467,])
        )
    }
}
