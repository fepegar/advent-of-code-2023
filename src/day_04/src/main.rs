use std::fs::read_to_string;

fn main() {
    println!("PART 1");
    println!("------");
    part_1("example.txt");
    part_1("input.txt");
    // println!("");
    // println!("PART 2");
    // println!("------");
    // part_2("example2.txt");
    // part_2("input.txt");
}


fn part_1(path: &str) {
    let file_content = read_to_string(path)
        .expect("Could not read file"
    );
    let lines = file_content.lines();
    let mut total_points = 0;
    for line in lines {
        let card = Card::from_line(line);
        total_points += card.get_points();
    }
    println!("{path}: {total_points}");
}


struct Card {
    id: u8,
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
            .parse::<u8>()
            .expect("Not a number"
        );
        let numbers_parts = parts[1].split(" | ").collect::<Vec<&str>>();
        let winning_numbers = Card::split_numbers(numbers_parts[0]);
        let numbers = Card::split_numbers(numbers_parts[1]);
        Card {
            id,
            winning_numbers,
            numbers,
        }
    }

    fn get_points(&self) -> u32 {
        let mut num_winning_numbers = 0;
        for number in &self.winning_numbers {
            if self.numbers.contains(number) {
                num_winning_numbers += 1;
            }
        }
        match num_winning_numbers {
            0 => 0,
            _ => 2u32.pow(num_winning_numbers - 1),
        }
    }
}
