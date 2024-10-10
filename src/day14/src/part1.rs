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
    fn test_get_load_before_square_rock() {
        assert_eq!(get_load_before_square_rock(4, 10), 30)
    }

    #[test]
    fn test_day14() {
        let input = read_to_string("data/test_day14.txt").unwrap();
        assert_eq!(calc_total_load(input), 136)
    }
}
