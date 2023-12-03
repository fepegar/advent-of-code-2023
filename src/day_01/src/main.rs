use std::fs::read_to_string;


const NUMBERS: [&str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];


fn main() {
    println!("PART 1");
    println!("------");
    part_1("example.txt");
    part_1("input.txt");
    println!("");
    println!("PART 2");
    println!("------");
    part_2("example2.txt");
    part_2("input.txt");
}


fn part_1(path: &str) {
    let file_content = read_file(path);
    let sum = sum_lines(&file_content);
    println!("{path}: {sum}");
}


fn read_file(path: &str) -> String {
    read_to_string(path).expect("Error reading file")
}


fn sum_lines(lines: &str) -> u32 {
    let mut sum = 0;
    for line in lines.lines() {
        let first_digit = get_first_digit(line).expect("No digits found in line");
        let last_digit = get_last_digit(line);
        let sum_line = 10 * first_digit + last_digit;
        sum += sum_line as u32;
    }
    sum
}


fn get_first_digit(line: &str) -> Option<u8> {
    for character in line.chars() {
        if character.is_digit(10) {
            return character.to_digit(10).map(|digit| digit as u8);
        }
    }
    None
}


fn get_last_digit(line: &str) -> u8 {
    let mut line_backwards = line.chars().rev();
    let last_digit_char = line_backwards
        .find(|character| character.is_digit(10))
        .expect("No digits found in line");
    let last_digit = last_digit_char.to_digit(10)
        .expect("Error converting digit to u32");
    last_digit as u8
}


fn part_2(path: &str) {
    let file_content = read_file(path);
    let sum = sum_lines_2(&file_content).expect("Error summing lines");
    println!("{path}: {sum}");
}


fn sum_lines_2(lines: &str) -> Option<u32> {
    let mut sum = 0;
    for line in lines.lines() {
        let first_digit = get_first_digit_2(line).expect("No digits found in line");
        let last_digit = get_last_digit_2(line).expect("No digits found in line");
        let sum_line = 10 * first_digit + last_digit;
        sum += sum_line as u32;
    }
    Some(sum)
}


fn get_first_digit_2(line: &str) -> Option<u8> {
    for first_index in 0..line.len() {
        let substring = &line[first_index..];
        let character = substring.chars().next()?;
        if character.is_digit(10) {
            let digit = character
                .to_digit(10)
                .map(|digit| digit as u8);
            return digit;
        } else {
            for (index, number) in NUMBERS.iter().enumerate() {
                if substring.starts_with(number) {
                    return Some(index as u8 + 1);
                }
            }
        }
    }
    None
}


fn get_last_digit_2(line: &str) -> Option<u8> {
    let mut result = None;
    for first_index in 0..line.len() {
        let substring = &line[first_index..];
        let character = substring
            .chars()
            .next()
            .expect("No characters found in line");
        if character.is_digit(10) {
            let digit = character
                .to_digit(10)
                .expect("Error converting digit to u32")
                as u8;
            result = Some(digit);
        } else {
            for (index, number) in NUMBERS.iter().enumerate() {
                if substring.starts_with(number) {
                    let digit = index + 1;
                    result = Some(digit as u8);
                }
            }
        }
    }
    result
}
