use std::fs::read_to_string;

#[derive(PartialEq, Clone, Copy)]
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

#[derive(Clone)]
pub struct Row {
    symbols: Vec<CellState>,
    load: i32,
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
        Self {
            symbols: states.clone(),
            load: states
                .clone()
                .iter()
                .enumerate()
                .filter(|(_, &x)| x == CellState::RoundedRock)
                .map(|(x, _)| x as i32)
                .sum(),
        }
    }

    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    pub fn tilt_in_right(&mut self) {
        let mut rounded_rock_counter = 0;
        let mut index = 0;
        let mut load = 0;
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
                        load += index - idx + 1;
                    }
                    index += 1;
                    rounded_rock_counter = 0;
                }
            }
        }
        if rounded_rock_counter != 0 {
            for idx in 1..=rounded_rock_counter {
                self.symbols[index - idx] = CellState::RoundedRock;
                load += index - idx + 1;
            }
        }
        self.load = load as i32;
    }
}

#[derive(PartialEq)]
enum PlatformFacingState {
    None = 0,
    North = 1,
    West = 2,
    South = 3,
    East = 4,
}

impl From<char> for PlatformFacingState {
    fn from(value: char) -> Self {
        match value {
            'N' => Self::North,
            'S' => Self::South,
            'E' => Self::East,
            'W' => Self::West,
            _ => panic!("Unknown character has been met. Consider using some char from the list - N, E, S, W.")
        }
    }
}

pub struct Platform {
    rows: Vec<Row>,
    state: PlatformFacingState,
}

impl FromIterator<Row> for Platform {
    fn from_iter<T: IntoIterator<Item = Row>>(iter: T) -> Self {
        Self {
            rows: iter.into_iter().collect(),
            state: PlatformFacingState::None,
        }
    }
}

impl From<Platform> for String {
    fn from(value: Platform) -> Self {
        value
            .rows
            .into_iter()
            .map(|row| format!("{}\n", String::from(row)))
            .collect()
    }
}

impl Platform {
    fn attempt_spin_cycle(&mut self) {
        if self.state == PlatformFacingState::None {
            self.tilt_north();
        }
        for state in vec![
            PlatformFacingState::West,
            PlatformFacingState::South,
            PlatformFacingState::East,
            PlatformFacingState::North,
        ]
        .into_iter()
        {
            self.rotate_clockwise();
            self.tilt_in_right();
            self.state = state;
        }
    }

    fn tilt_in_right(&mut self) {
        self.rows.iter_mut().for_each(|row| row.tilt_in_right());
    }

    fn rotate_clockwise(&mut self) {
        let mut new_rows: Vec<Vec<CellState>> = vec![vec![]; self.rows[0].len()];
        for row in self.rows.iter().rev() {
            for (id, elem) in row.symbols.iter().enumerate() {
                new_rows[id].push(elem.clone());
            }
        }
        self.rows = new_rows.into_iter().map(|x| Row::new(x)).collect()
    }

    pub fn tilt_north(&mut self) {
        self.rotate_clockwise();
        self.tilt_in_right();
        self.state = PlatformFacingState::North;
    }

    pub fn calc_load(&self) -> i32 {
        self.rows.iter().map(|row| row.load).sum()
    }
}

pub fn part1(path: &str) -> i32 {
    let input = read_to_string(path).unwrap();
    let mut platform = Platform::from_iter(input.lines().map(|line| Row::from(line.to_string())));
    platform.tilt_north();
    platform.calc_load()
}

pub fn part2(path: &str) -> i32 {
    let input = read_to_string(path).unwrap();
    let mut platform = Platform::from_iter(input.lines().map(|line| Row::from(line.to_string())));
    let mut load_keeper = platform.calc_load();
    let mut iteration_keeper = 1;
    for iteration in 0..1000000000 {
        platform.attempt_spin_cycle();
        let load = platform.calc_load();
        if load != load_keeper {
            load_keeper = load;
            iteration_keeper = 1;
        } else {
            iteration_keeper += 1
        }
        if iteration_keeper >= 10 {
            return platform.calc_load();
        }
        if iteration % 10 == 0 {
            println!("{} iterations passed", iteration)
        }
    }
    platform.calc_load()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_row_tilt_in_right() {
        let mut row = Row::from("O....#....".to_string());
        row.tilt_in_right();
        assert_eq!(
            <Row as Into<String>>::into(row.clone()),
            "....O#....".to_string()
        );
        assert_eq!(row.load, 5)
    }

    #[test]
    fn test_row_tilt_in_rigth_one_more() {
        let mut row = Row::from("OO.#O....O".to_string());
        row.tilt_in_right();
        assert_eq!(
            <Row as Into<String>>::into(row.clone()),
            ".OO#....OO".to_string()
        );
        assert_eq!(row.load, 24);
    }

    #[test]
    fn test_platform_rotate_clockwise() {
        let input = read_to_string("test_data.txt").unwrap();
        let rotated_input = "##..O.O.OO\n\
            O....OO...\n\
            O..O#...O.\n\
            ......#.O.\n\
            ......O.#.\n\
            ##.#O..#.#\n\
            .#.O...#..\n\
            .#O.#O....\n\
            .....#....\n\
            ...O#.O.#.\n"
            .to_string();
        let mut platform =
            Platform::from_iter(input.lines().map(|line| Row::from(line.to_string())));
        platform.rotate_clockwise();
        assert_eq!(String::from(platform), rotated_input)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("test_data.txt"), 136)
    }
}
