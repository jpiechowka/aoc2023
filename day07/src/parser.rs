use std::ops::Deref;

use itertools::Itertools;

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub hand_type: HandType,
    pub bid: u64,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub enum Card {
    Joker = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    pub fn new_part1(cards: Vec<Card>, bid: u64) -> Self {
        let hand_type = Self::get_hand_type_part1(&cards);
        Hand {
            cards,
            hand_type,
            bid,
        }
    }

    pub fn new_part2(cards: Vec<Card>, bid: u64) -> Self {
        let hand_type = Self::get_hand_type_part2(&cards);
        Hand {
            cards,
            hand_type,
            bid,
        }
    }

    fn get_hand_type_part1(cards: &Vec<Card>) -> HandType {
        let card_counts = cards.iter().counts().values().sorted().join("|");
        match card_counts.deref() {
            "1" => HandType::HighCard,
            "2" => HandType::OnePair,
            "3" => HandType::ThreeOfAKind,
            "4" => HandType::FourOfAKind,
            "5" => HandType::FiveOfAKind,
            "1|1" => HandType::HighCard,
            "1|2" => HandType::OnePair,
            "1|3" => HandType::ThreeOfAKind,
            "1|4" => HandType::FourOfAKind,
            "2|2" => HandType::TwoPair,
            "2|3" => HandType::FullHouse,
            "1|1|3" => HandType::ThreeOfAKind,
            "1|2|2" => HandType::TwoPair,
            "1|1|2" => HandType::OnePair,
            "1|1|1" => HandType::HighCard,
            "1|1|1|1" => HandType::HighCard,
            "1|1|1|2" => HandType::OnePair,
            "1|1|1|1|1" => HandType::HighCard,
            val => panic!("unable to get hand type. Encountered: {}", val),
        }
    }

    fn get_hand_type_part2(cards: &Vec<Card>) -> HandType {
        let cards_without_joker: Vec<Card> = cards
            .iter()
            .filter(|&card| *card != Card::Joker)
            .cloned()
            .collect();
        let jokers_count = cards.len() - cards_without_joker.len();

        if jokers_count == 5usize || jokers_count == 4usize {
            return HandType::FiveOfAKind;
        }

        let hand_type_no_joker = Hand::get_hand_type_part1(&cards_without_joker);

        match jokers_count {
            3usize => match hand_type_no_joker {
                HandType::OnePair => HandType::FiveOfAKind,
                HandType::HighCard => HandType::FourOfAKind,
                _ => panic!("unable to determine hand type with 3 jokers"),
            },
            2usize => match hand_type_no_joker {
                HandType::ThreeOfAKind => HandType::FiveOfAKind,
                HandType::OnePair => HandType::FourOfAKind,
                HandType::HighCard => HandType::ThreeOfAKind,
                _ => panic!("unable to determine hand type with 2 jokers"),
            },
            1usize => match hand_type_no_joker {
                HandType::FourOfAKind => HandType::FiveOfAKind,
                HandType::ThreeOfAKind => HandType::FourOfAKind,
                HandType::TwoPair => HandType::FullHouse,
                HandType::OnePair => HandType::ThreeOfAKind,
                HandType::HighCard => HandType::OnePair,
                _ => panic!("unable to determine hand type with 1 joker"),
            },
            0usize => hand_type_no_joker,
            _ => panic!("unable to determine hand type with jokers"),
        }
    }
}

pub fn parse_input_part1(input: &str) -> Vec<Hand> {
    input.lines().map(parse_hand_part1).collect()
}

pub fn parse_input_part2(input: &str) -> Vec<Hand> {
    input.lines().map(parse_hand_part2).collect()
}

fn parse_hand_part1(input: &str) -> Hand {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let cards_str = parts[0];
    let bid: u64 = parts[1].parse().expect("should parse bid");
    let cards: Vec<Card> = cards_str.chars().map(parse_card_rank_part1).collect();
    Hand::new_part1(cards, bid)
}

fn parse_hand_part2(input: &str) -> Hand {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let cards_str = parts[0];
    let bid: u64 = parts[1].parse().expect("should parse bid");
    let cards: Vec<Card> = cards_str.chars().map(parse_card_rank_part2).collect();
    Hand::new_part2(cards, bid)
}

fn parse_card_rank_part1(card: char) -> Card {
    match card {
        'A' => Card::Ace,
        'K' => Card::King,
        'Q' => Card::Queen,
        'J' => Card::Jack,
        'T' => Card::Ten,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        _ => panic!("invalid card rank encountered"),
    }
}

fn parse_card_rank_part2(card: char) -> Card {
    match card {
        'A' => Card::Ace,
        'K' => Card::King,
        'Q' => Card::Queen,
        'T' => Card::Ten,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        'J' => Card::Joker,
        _ => panic!("invalid card rank encountered"),
    }
}
