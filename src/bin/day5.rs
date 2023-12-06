fn main() {}

struct Seed(u64);
struct Ranges(u64, u64, u64, u64);

struct Category(Vec<Ranges>);

fn parse_seeds(line: &str) -> Vec<Seed> {
    line.split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .into_iter()
        .map(|x| Seed(x.parse::<u64>().unwrap()))
        .collect::<Vec<Seed>>()
}

fn parse_category(lines: Vec<String>) -> Category {
    Category(
        lines
            .iter()
            .map(|&line| {
                line.split_whitespace()
                    .into_iter()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>()
            })
            .map(|x| Ranges(x[0], x[0] + x[2], x[1], x[1] + x[2]))
            .collect(),
    )
}

fn parse_almanac(data: &str) -> (Vec<Seed>, Vec<Category>) {
    let mut lines = data.lines();
    let mut categories: Vec<Category> = Vec::new();
    let seeds = parse_seeds(lines.next().unwrap());
    while let Some(line) = lines.next() {
        lines.next().unwrap();
        lines.next().unwrap();
        let numbers = lines
            .take_while(|line| line.chars().next().unwrap().is_ascii_digit())
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        categories.push(parse_category(numbers));
    }
    (seeds, categories)
}

fn find_lowest_location_number(seeds: Vec<Seed>, categories: Vec<Seed>) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_test_example() {
        assert_eq!(35, 35)
    }
}
