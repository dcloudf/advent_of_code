use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let file = read_to_string("./data/day7.txt").unwrap();
    let cards = parse_card_hands(&file, false);
    println!("{}", calc_winnings(cards));
    let cards = parse_card_hands(&file, true);
    println!("{}", calc_winnings(cards));
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
enum CardValue {
    JOKER = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
    NINE = 9,
    TEN = 10,
    JACK = 11,
    QUEEN = 12,
    KING = 13,
    ACE = 14,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum CombinationValue {
    HighCard = 1,
    Pair = 2,
    TwoPair = 3,
    ThreeOfKind = 4,
    FullHouse = 5,
    FourOfKind = 6,
    FiveOfKind = 7,
}

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
struct CardHand {
    combination: CombinationValue,
    hand: Vec<CardValue>,
    bid: u32,
}

impl CardHand {
    fn new(input: &str, use_joker: bool) -> Self {
        let mut iter = input.split_whitespace();
        let (card_literals, bid) = (
            iter.next().unwrap(),
            iter.next().unwrap().parse::<u32>().unwrap(),
        );
        let hand: Vec<CardValue> = card_literals
            .chars()
            .map(|char| match char {
                'A' => CardValue::ACE,
                'K' => CardValue::KING,
                'Q' => CardValue::QUEEN,
                'J' => match use_joker {
                    true => CardValue::JOKER,
                    false => CardValue::JACK,
                },
                'T' => CardValue::TEN,
                '9' => CardValue::NINE,
                '8' => CardValue::EIGHT,
                '7' => CardValue::SEVEN,
                '6' => CardValue::SIX,
                '5' => CardValue::FIVE,
                '4' => CardValue::FOUR,
                '3' => CardValue::THREE,
                '2' => CardValue::TWO,
                _ => panic!("Incorrect char"),
            })
            .collect();
        let combination = match use_joker {
            true => calc_combination_with_joker(&hand),
            false => calc_combination(&hand),
        };
        CardHand {
            combination,
            hand,
            bid,
        }
    }
}

fn parse_card_hands(input: &str, use_jokers: bool) -> Vec<CardHand> {
    input
        .lines()
        .map(|line| CardHand::new(line, use_jokers))
        .collect::<Vec<CardHand>>()
}

fn match_counter_to_combination(counter: &HashMap<&CardValue, i32>) -> CombinationValue {
    match counter.values().max().unwrap() {
        5 => CombinationValue::FiveOfKind,
        4 => CombinationValue::FourOfKind,
        3 => match counter.values().min().unwrap() {
            2 => CombinationValue::FullHouse,
            _ => CombinationValue::ThreeOfKind,
        },
        2 => match counter.values().filter(|x| **x == 2).count() {
            1 => CombinationValue::Pair,
            2 => CombinationValue::TwoPair,
            _ => panic!("Incorrect card has been met"),
        },
        _ => CombinationValue::HighCard,
    }
}

fn calc_combination(cards: &[CardValue]) -> CombinationValue {
    let mut counter = HashMap::new();
    for card in cards.into_iter() {
        counter.entry(card).and_modify(|x| *x += 1).or_insert(1);
    }
    match_counter_to_combination(&counter)
}

fn calc_combination_with_joker(cards: &[CardValue]) -> CombinationValue {
    let mut counter = HashMap::new();
    for card in cards.into_iter() {
        counter.entry(card).and_modify(|x| *x += 1).or_insert(1);
    }
    if counter.get(&CardValue::JOKER) == None || counter.get(&CardValue::JOKER) == Some(&5) {
        return match_counter_to_combination(&counter);
    }
    let joker_value = counter[&CardValue::JOKER];
    let max_value = counter
        .iter()
        .filter(|(&x, _y)| x != &CardValue::JOKER)
        .map(|(_x, y)| y)
        .max()
        .unwrap();
    let max_key = counter
        .iter()
        .filter(|(&x, y)| x != &CardValue::JOKER && y == &max_value)
        .next()
        .unwrap()
        .0;
    counter
        .entry(&max_key)
        .and_modify(|x| *x += joker_value.to_owned());
    counter.entry(&CardValue::JOKER).and_modify(|x| *x = 0);
    match_counter_to_combination(&counter)
}

fn calc_winnings(mut cards: Vec<CardHand>) -> u32 {
    cards.sort();
    cards
        .iter()
        .enumerate()
        .map(|(idx, card)| (idx + 1) as u32 * card.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7_test_example() {
        let example = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        let card_hands: Vec<CardHand> = parse_card_hands(example, false);
        assert_eq!(calc_winnings(card_hands), 6440);

        let card_hands: Vec<CardHand> = parse_card_hands(example, true);
        assert_eq!(calc_winnings(card_hands), 5905);
    }

    #[test]
    fn test_ord_card_hand() {
        let first_hand = CardHand {
            hand: vec![
                CardValue::THREE,
                CardValue::THREE,
                CardValue::THREE,
                CardValue::THREE,
                CardValue::TWO,
            ],
            combination: CombinationValue::FourOfKind,
            bid: 1,
        };
        let second_hand = CardHand {
            hand: vec![
                CardValue::TWO,
                CardValue::ACE,
                CardValue::ACE,
                CardValue::ACE,
                CardValue::ACE,
            ],
            combination: CombinationValue::FourOfKind,
            bid: 2,
        };
        assert_eq!(first_hand > second_hand, true);

        let third_hand = CardHand {
            hand: vec![
                CardValue::THREE,
                CardValue::THREE,
                CardValue::THREE,
                CardValue::THREE,
                CardValue::TWO,
            ],
            combination: CombinationValue::FourOfKind,
            bid: 3,
        };
        assert_eq!(third_hand > first_hand, true);
    }
}
