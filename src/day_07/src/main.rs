use std::collections::HashMap;
use std::iter::zip;
use std::cmp::Ordering;

static mut PART: u32 = 1;

fn main() {
    println!("PART 1");
    println!("------");
    part_1("example.txt");
    part_1("input.txt");
}

fn part_1(filename: &str) {
    unsafe { PART = 1 };
    let hands = read_hands(filename);
    let mut total = 0;
    for (rank, hand) in hands.iter().enumerate() {
        total += hand.bid * (rank + 1) as u32;
    }
    println!("{filename}: {total}");
}

fn read_hands(filename: &str) -> Vec<Hand> {
    let mut hands = Vec::new();
    let contents = std::fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    for line in contents.lines() {
        let hand = Hand::from_line(line);
        hands.push(hand);
    }
    hands.sort();
    hands
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn from_line(line: &str) -> Hand {
        let mut cards = Vec::new();
        let mut bid = 0;
        let mut is_bid = false;
        for character in line.chars() {
            if character == ' ' {
                is_bid = true;
                continue;
            }
            if !is_bid {
                let card = Card { symbol: character };
                cards.push(card);
            } else {
                bid = bid * 10 + character.to_digit(10).unwrap();
            }
        }
        if cards.len() != 5 {
            panic!("Invalid number of cards");
        }
        Hand { cards, bid }
    }

    fn get_type(&self) -> HandType {
        let mut card_count = HashMap::new();
        for card in self.cards.iter() {
            let count = card_count.entry(card.symbol).or_insert(0);
            *count += 1;
        }
        let mut sorted_card_count: Vec<u32> = card_count.iter().map(|(_, count)| *count).collect();
        sorted_card_count.sort_unstable_by(|a, b| b.cmp(a));

        let card_count_vec: Vec<u32> = sorted_card_count.into_iter().collect();
        match card_count_vec.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("Invalid card count: {:?}", card_count_vec),
        }
    }

    fn get_value(&self) -> u32 {
        let mut hand_to_value = HashMap::new();
        hand_to_value.insert(HandType::HighCard, 0);
        hand_to_value.insert(HandType::OnePair, 1);
        hand_to_value.insert(HandType::TwoPair, 2);
        hand_to_value.insert(HandType::ThreeOfAKind, 3);
        hand_to_value.insert(HandType::FullHouse, 4);
        hand_to_value.insert(HandType::FourOfAKind, 5);
        hand_to_value.insert(HandType::FiveOfAKind, 6);
        let hand_type = self.get_type();
        let hand_value = hand_to_value.get(&hand_type).unwrap();
        *hand_value
    }

    fn compare_full(&self, other: &Self) -> Ordering {
        let self_value = self.get_value();
        let other_value = other.get_value();
        if self_value > other_value {
            return Ordering::Greater;
        } else if self_value < other_value {
            return Ordering::Less;
        } else {
            let zipped = zip(self.cards.iter(), other.cards.iter());

            for (self_card, other_card) in zipped {
                if self_card > other_card {
                    return Ordering::Greater;
                } else if self_card < other_card {
                    return Ordering::Less;
                } else {
                    continue;
                }
            }
            panic!();
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare_full(&other)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}

impl Eq for Hand {}

#[derive(Clone, Debug)]
struct Card {
    symbol: char,
}

impl Card {
    fn get_value(&self) -> u32 {
        let mut char_to_value = HashMap::new();
        char_to_value.insert('2', 2);
        char_to_value.insert('3', 3);
        char_to_value.insert('4', 4);
        char_to_value.insert('5', 5);
        char_to_value.insert('6', 6);
        char_to_value.insert('7', 7);
        char_to_value.insert('8', 8);
        char_to_value.insert('9', 9);
        char_to_value.insert('T', 10);
        char_to_value.insert('J', unsafe { Card::get_j_value() } );
        char_to_value.insert('Q', 12);
        char_to_value.insert('K', 13);
        char_to_value.insert('A', 14);
        *char_to_value.get(&self.symbol).unwrap()
    }

    unsafe fn get_j_value() -> u32 {
        match PART {
            1 => 11,
            2 => 1,
            _ => panic!("Invalid part"),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_value().cmp(&other.get_value())
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}

impl Eq for Card {}

#[derive(Debug, PartialEq, Eq, Hash)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

unsafe fn part_2(filename: &str) {
    unsafe { PART = 2 };
}

impl Hand {
    fn get_best_type(&self) -> HandType {
        // Get possible type when replacing J with other cards
        if !self.has_j() {
            return self.get_type();
        }
        let mut best_type = self.get_type();
        let possible_hands = Vec::new();
        let cards = self.cards.clone();
        cards.sort();
    }

    fn has_j(&self) -> bool {
        for card in self.cards.iter() {
            if card.symbol == 'J' {
                return true;
            }
        }
        false
    }
}
