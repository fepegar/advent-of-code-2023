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
    part_1();
    part_2();
}


fn part_1() {
    println!("Part 1:");
    print_sum("example.txt");
    print_sum("input.txt");
}


fn print_sum(path: &str) {
    let file_content = read_to_string(path).unwrap();
    let sum = sum_lines(&file_content).unwrap();
    println!("{path}: {sum}");
}


fn sum_lines(lines: &str) -> Option<u32> {
    let mut sum = 0;
    for line in lines.lines() {
        let first_digit = get_first_digit(line)?;
        let last_digit = get_last_digit(line)?;
        let first_digit = first_digit.to_digit(10)?;
        let last_digit = last_digit.to_digit(10)?;
        let sum_line = 10 * first_digit + last_digit;
        sum += sum_line;
    }
    Some(sum)
}


fn get_first_digit(line: &str) -> Option<char> {
    for character in line.chars() {
        if character.is_digit(10) {
            return Some(character);
        }
    }
    None
}


fn get_last_digit(line: &str) -> Option<char> {
    line.chars().rev().find(|character| character.is_digit(10))
}


fn part_2() {
    println!("Part 2:");
    print_sum_2("example2.txt");
    print_sum_2("input.txt");
}


fn print_sum_2(path: &str) {
    let file_content = read_to_string(path).unwrap();
    let sum = sum_lines_2(&file_content).unwrap();
    println!("{path}: {sum}");
}


fn sum_lines_2(lines: &str) -> Option<u32> {
    let mut sum = 0;
    for line in lines.lines() {
        let first_digit = get_first_digit_2(line)?;
        let last_digit = get_last_digit_2(line)?;
        let first_digit = first_digit.to_digit(10)?;
        let last_digit = last_digit.to_digit(10)?;
        let sum_line = 10 * first_digit + last_digit;
        sum += sum_line;
    }
    Some(sum)
}


fn get_first_digit_2(line: &str) -> Option<char> {
    for first_index in 0..line.len() {
        let substring = &line[first_index..];
        let character = substring.chars().next()?;
        if character.is_digit(10) {
            return Some(character);
        } else {
            for (index, number) in NUMBERS.iter().enumerate() {
                if substring.starts_with(number) {
                    return Some(std::char::from_digit(index as u32 + 1, 10)?);
                }
            }
        }
    }
    None
}


fn get_last_digit_2(line: &str) -> Option<char> {
    let mut result: Option<char> = None;
    for first_index in 0..line.len() {
        let substring = &line[first_index..];
        let character = substring.chars().next()?;
        if character.is_digit(10) {
            result = Some(character);
        } else {
            for (index, number) in NUMBERS.iter().enumerate() {
                if substring.starts_with(number) {
                    result = Some(std::char::from_digit(index as u32 + 1, 10)?);
                }
            }
        }
    }
    result
}
