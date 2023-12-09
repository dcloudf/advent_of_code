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
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
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
                'A' => CardValue::Ace,
                'K' => CardValue::King,
                'Q' => CardValue::Queen,
                'J' => match use_joker {
                    true => CardValue::Joker,
                    false => CardValue::Jack,
                },
                'T' => CardValue::Ten,
                '9' => CardValue::Nine,
                '8' => CardValue::Eight,
                '7' => CardValue::Seven,
                '6' => CardValue::Six,
                '5' => CardValue::Five,
                '4' => CardValue::Four,
                '3' => CardValue::Three,
                '2' => CardValue::Two,
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
    for card in cards.iter() {
        counter.entry(card).and_modify(|x| *x += 1).or_insert(1);
    }
    match_counter_to_combination(&counter)
}

fn calc_combination_with_joker(cards: &[CardValue]) -> CombinationValue {
    let mut counter = HashMap::new();
    for card in cards.iter() {
        counter.entry(card).and_modify(|x| *x += 1).or_insert(1);
    }
    if counter.get(&CardValue::Joker).is_none() || counter.get(&CardValue::Joker) == Some(&5) {
        return match_counter_to_combination(&counter);
    }
    let joker_value = counter[&CardValue::Joker];
    let max_value = counter
        .iter()
        .filter(|(&x, _y)| x != &CardValue::Joker)
        .map(|(_x, y)| y)
        .max()
        .unwrap();
    let max_key = counter
        .iter().find(|(&x, y)| x != &CardValue::Joker && y == &max_value)
        .unwrap()
        .0;
    counter
        .entry(max_key)
        .and_modify(|x| *x += joker_value.to_owned());
    counter.entry(&CardValue::Joker).and_modify(|x| *x = 0);
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
                CardValue::Three,
                CardValue::Three,
                CardValue::Three,
                CardValue::Three,
                CardValue::Two,
            ],
            combination: CombinationValue::FourOfKind,
            bid: 1,
        };
        let second_hand = CardHand {
            hand: vec![
                CardValue::Two,
                CardValue::Ace,
                CardValue::Ace,
                CardValue::Ace,
                CardValue::Ace,
            ],
            combination: CombinationValue::FourOfKind,
            bid: 2,
        };
        assert_eq!(first_hand > second_hand, true);

        let third_hand = CardHand {
            hand: vec![
                CardValue::Three,
                CardValue::Three,
                CardValue::Three,
                CardValue::Three,
                CardValue::Two,
            ],
            combination: CombinationValue::FourOfKind,
            bid: 3,
        };
        assert_eq!(third_hand > first_hand, true);
    }
}
