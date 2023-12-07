use std::collections::{HashSet, HashMap};
use std::fs::read_to_string;

fn main() {
    let file = read_to_string("./data/day4.txt").unwrap();
    println!("{}", file.lines().map(|x| parse_scratchcard(x)).map(|x| points_for_wins(&x)).sum::<u32>())
}

#[derive(Debug, PartialEq)]
struct Scratchcard {
    id: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

fn parse_scratchcard(line: &str) -> Scratchcard {
    let mut splitter = line.split(':');
    let (card_info, numbers) = (
        splitter.next().unwrap(),
        splitter.next().unwrap(),
        );
    splitter = numbers.split('|');
    let (win_nums, my_nums) = (
        splitter.next().unwrap(),
        splitter.next().unwrap(),
        );
    Scratchcard{
        id: card_info.split_whitespace().nth(1).unwrap().parse::<u32>().unwrap(),
        winning_numbers: win_nums.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect(),
        my_numbers: my_nums.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect(),
    }
}

fn points_for_wins(card: &Scratchcard) -> u32 {
    let wins_set: HashSet<u32> = HashSet::from_iter(card.winning_numbers.iter().cloned());
    let my_nums_set: HashSet<u32> = HashSet::from_iter(card.my_numbers.iter().cloned());
    let intersections: Vec<u32> = wins_set.intersection(&my_nums_set).cloned().collect();
    intersections.len() as u32
}

fn count_duplicates(card: &Scratchcard, copy_map: &mut HashMap<u32, u32>) {
    let amount = points_for_wins(card);
    let times = copy_map.get(&card.id).copied().unwrap();
    for id in card.id + 1..card.id + amount + 1 {
        println!("{} {}", id, amount);
        copy_map.entry(id).and_modify(|x| *x += times);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_scratchcard() {
        assert_eq!(parse_scratchcard("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), Scratchcard{id: 1, winning_numbers: vec![41, 48, 83, 86, 17], my_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]})
    }

    #[test]
    fn test_points_for_wins() {
        assert_eq!(points_for_wins(&Scratchcard{id: 1, winning_numbers: vec![41, 48, 83, 86, 17], my_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]}), 8)
    }

    #[test]
    fn test_count_duplicates() {
        let file = read_to_string("./data/test_day4.txt").unwrap();
        let cards: Vec<Scratchcard> = file.lines().map(|x| parse_scratchcard(x)).collect();
        println!("{:?}", cards);
        assert_eq!(cards.iter().map(|x| points_for_wins(x)).collect::<Vec<u32>>(), Vec::from([4, 2, 2, 1, 0, 0]));
        let mut counter: HashMap<u32, u32> = HashMap::new();
        for card in cards.iter() {
            counter.insert(card.id, 1);
        }
        for card in cards.iter() {
            count_duplicates(card, &mut counter)
        }
        assert_eq!(counter, HashMap::from([(1, 1), (2, 2), (3, 4), (4, 8), (5, 14), (6, 1)]))
    }
}