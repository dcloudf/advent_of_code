enum CellState {
    Empty,
    RoundedRock,
    CubeShapedRock,
}

impl From<char> for CellState {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            'O' => Self::RoundedRock,
            '#' => Self::CubeShapedRock,
            _ => panic!("Unknown char has ben met. Consider checking the input"),
        }
    }
}

impl From<CellState> for char {
    fn from(value: CellState) -> Self {
        match value {
            CellState::Empty => '.',
            CellState::RoundedRock => 'O',
            CellState::CubeShapedRock => '#',
        }
    }
}

pub struct Row {
    symbols: Vec<CellState>,
}

impl From<String> for Row {
    fn from(value: String) -> Self {
        Row::new(value.chars().map(|c| c.into()).collect())
    }
}

impl From<Row> for String {
    fn from(row: Row) -> Self {
        row.symbols
            .into_iter()
            .map(|s| <CellState as Into<char>>::into(s))
            .collect()
    }
}

impl Row {
    fn new(states: Vec<CellState>) -> Self {
        Self { symbols: states }
    }

    pub fn tilt_in_right(&mut self) {
        let mut rounded_rock_counter = 0;
        let mut index = 0;
        while index < self.symbols.len() {
            match self.symbols[index] {
                CellState::Empty => index += 1,
                CellState::RoundedRock => {
                    self.symbols[index] = CellState::Empty;
                    index += 1;
                    rounded_rock_counter += 1;
                }
                CellState::CubeShapedRock => {
                    for idx in 1..=rounded_rock_counter {
                        self.symbols[index - idx] = CellState::RoundedRock;
                    }
                    index += 1;
                    rounded_rock_counter = 0;
                }
            }
        }
    }
}

enum PlatformFacingState {
    North,
    East,
    South,
    West,
}

pub struct Platform {
    rows: Vec<Row>,
    state: PlatformFacingState,
}

impl Platform {
    fn attempt_spin_cycle(&self) {}

    fn tilt_in_right(&self, direction: char) {}

    fn rotate_right(&self) {}

    fn calc_load_on_side_beams(&self, side: char) -> i32 {
        return 42;
    }
}

pub fn calc_total_load(input: String) -> i32 {
    let mut load = 0;
    let mut lines = input.lines().rev();
    let mut rounded_rocks: Vec<i32> = lines
        .next()
        .unwrap()
        .chars()
        .map(|x| match x {
            'O' => 1,
            _ => 0,
        })
        .collect();
    for (row_id, line) in lines.enumerate() {
        rounded_rocks = line
            .chars()
            .zip(rounded_rocks.iter())
            .map(|(x, y)| match x {
                '.' => *y,
                'O' => y + 1,
                '#' => {
                    load += get_load_before_square_rock(*y, row_id + 2);
                    0
                }
                _ => panic!("Unexpected symbol has been meet"),
            })
            .collect();
    }
    load + rounded_rocks
        .iter()
        .map(|x| match x {
            0 => 0,
            n => get_load_before_square_rock(
                *n,
                input.lines().map(|_| 1).sum::<i32>() as usize + 1usize,
            ),
        })
        .sum::<i32>()
}

fn get_load_before_square_rock(rock_num: i32, row_id: usize) -> i32 {
    (((row_id as i32) - rock_num)..row_id as i32).sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_row_tilt_in_right() {
        let mut row = Row::from("O....#....".to_string());
        row.tilt_in_right();
        assert_eq!(<Row as Into<String>>::into(row), "....O#....".to_string())
    }

    #[test]
    fn test_get_load_before_square_rock() {
        assert_eq!(get_load_before_square_rock(4, 10), 30)
    }

    #[test]
    fn test_day14() {
        let input = read_to_string("data/test_day14.txt").unwrap();
        assert_eq!(calc_total_load(input), 136)
    }
}
