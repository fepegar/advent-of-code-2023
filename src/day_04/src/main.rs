use std::fs::read_to_string;

fn main() {
    println!("PART 1");
    println!("------");
    part_1("example.txt");
    part_1("input.txt");
    println!("");
    println!("PART 2");
    println!("------");
    part_2("example.txt");
    part_2("input.txt");
}

fn part_1(path: &str) {
    let cards = read_cards(path);
    let mut total_points = 0;
    for card in cards {
        total_points += card.get_points();
    }
    println!("{path}: {total_points}");
}

fn read_cards(path: &str) -> Vec<Card> {
    let file_content = read_to_string(path)
        .expect("Could not read file"
    );
    let lines = file_content.lines();
    let mut cards = Vec::new();
    for line in lines {
        cards.push(Card::from_line(line));
    }
    cards
}

#[derive(Clone)]
struct Card {
    _id: u32,
    winning_numbers: Vec<u8>,
    numbers: Vec<u8>,
}

impl Card {
    fn split_numbers(numbers: &str) -> Vec<u8> {
        numbers
            .split_whitespace()
            .map(|n| n.parse::<u8>().expect("Not a number"))
            .collect::<Vec<u8>>()
    }

    fn from_line(line: &str) -> Card {
        let parts = line.split(":").collect::<Vec<&str>>();
        let id = parts[0]
            .split_whitespace()
            .nth(1)
            .expect("No id")
            .parse::<u32>()
            .expect("Not a number");
        let numbers_parts = parts[1].split(" | ").collect::<Vec<&str>>();
        let winning_numbers = Card::split_numbers(numbers_parts[0]);
        let numbers = Card::split_numbers(numbers_parts[1]);
        Card {
            _id: id,
            winning_numbers,
            numbers,
        }
    }

    fn get_num_matches(&self) -> u32 {
        let mut num_matches = 0;
        for number in &self.winning_numbers {
            if self.numbers.contains(number) {
                num_matches += 1;
            }
        }
        num_matches
    }

    fn get_points(&self) -> u32 {
        let num_matches = self.get_num_matches();
        match num_matches {
            0 => 0,
            _ => 2u32.pow(num_matches - 1),
        }
    }
}

fn part_2(path: &str) {
    let cards = read_cards(path);
    let won_cards = get_won_cards(cards);
    println!("{path}: {num_won_cards}", num_won_cards=won_cards.len());
}

fn get_won_cards(cards: Vec<Card>) -> Vec<Card> {
    let mut won_cards = Vec::new();
    for card in &cards {
        won_cards.push(card.clone());
        let new_cards = card.get_won_cards(&cards);
        won_cards.extend(new_cards);
    }
    won_cards
}

impl Card {
    fn get_won_cards(&self, cards: &Vec<Card>) -> Vec<Card> {
        let mut won_cards = Vec::new();
        let num_matches = self.get_num_matches();
        let id_ini = self._id + 1;
        let id_fin = id_ini + num_matches;
        let range = id_ini..id_fin;
        for card_id in range {
            let index = card_id - 1;
            let card = &cards[index as usize];
            won_cards.push(card.clone());
            let card_won_cards = card.get_won_cards(&cards);
            won_cards.extend(card_won_cards);
        }
        won_cards
    }
}
