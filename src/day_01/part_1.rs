fn main() {
    print_sum("example.txt");
    print_sum("input.txt");
}


fn print_sum(path: &str) {
    let file_content = std::fs::read_to_string(path).unwrap();
    let sum = sum_lines(&file_content).unwrap();
    println!("{sum}");
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
