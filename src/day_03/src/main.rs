use std::cmp;
use std::string;

fn main() {
    println!("PART 1");
    println!("------");
    part_1("example.txt");
    part_1("input.txt");
}


fn part_1(filename: &str) {
    let contents = read_file(filename);
    let schematic = get_schematic(contents);
    let numbers = get_numbers_from_schematic(&schematic);
    let mut sum = 0;
    for number in numbers {
        println!("{value}", value=number.value);
        println!("Is part?");
        if is_part(&number, &schematic) {
            sum += number.value;
            println!("Yes");
        } else {
            println!("No");
        }
        println!("");
    }
    println!("{filename}: {sum}");
}


fn read_file(filename: &str) -> string::String {
    let contents = std::fs::read_to_string(filename).expect("Error reading file");
    contents
}


fn get_schematic(contents: string::String) -> Vec<Vec<char>> {
    let mut schematic: Vec<Vec<char>> = Vec::new();
    let lines = contents.lines();
    for line in lines {
        let mut row: Vec<char> = Vec::new();
        for character in line.chars() {
            row.push(character);
        }
        schematic.push(row);
    }
    schematic
}


fn get_numbers_from_schematic(schematic: &Vec<Vec<char>>) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();
    let mut col_ini = 0;
    for (row, line) in schematic.iter().enumerate() {
        let mut current_digits = Vec::new();
        for (col, character) in line.iter().enumerate() {
            if character.is_digit(10) {
                if current_digits.len() == 0 {
                    col_ini = col;
                }
                current_digits.push(character.to_digit(10).unwrap());
            } else if current_digits.len() > 0 {
                let value = get_value_from_digits(current_digits);
                let number = Number {
                    row: row as i32,
                    col_ini: col_ini as i32,
                    col_fin: col as i32,
                    value: value,
                };
                numbers.push(number);

                current_digits = Vec::new();
            }
        }
        if current_digits.len() > 0 {
            let value = get_value_from_digits(current_digits);
            let col = line.len();
            let number = Number {
                row: row as i32,
                col_ini: col_ini as i32,
                col_fin: col as i32,
                value: value,
            };
            numbers.push(number);
        }
    }
    numbers
}


fn get_value_from_digits(digits: Vec<u32>) -> u32 {
    let mut value = 0;
    for (i, digit) in digits.iter().rev().enumerate() {
        value += digit * (10 as u32).pow(i as u32);
    }
    value
}


struct Number {
    row: i32,
    col_ini: i32,
    col_fin: i32,
    value: u32,
}


fn is_part(number: &Number, schematic: &Vec<Vec<char>>) -> bool {
    let num_rows = schematic.len() as i32;
    let num_cols = schematic[0].len() as i32;
    let row_ini = cmp::max(number.row - 1, 0);
    let row_fin = cmp::min(number.row + 1, num_rows - 1);
    for row in row_ini..=row_fin {
        let col_ini = cmp::max(number.col_ini - 1, 0);
        let col_fin = cmp::min(number.col_fin + 1, num_cols - 1);
        println!("number row: {row_ini}", row_ini=number.row);
        println!("number col_ini: {col_ini}", col_ini=number.col_ini);
        println!("number col_fin: {col_fin}", col_fin=number.col_fin);
        println!("row_ini: {row_ini}", row_ini=row_ini);
        println!("row_fin: {row_fin}", row_fin=row_fin);
        println!("col_ini: {col_ini}", col_ini=col_ini);
        println!("col_fin: {col_fin}", col_fin=col_fin);
        for col in col_ini..=col_fin {
            let value = schematic[row as usize][col as usize];
            println!("value in row {row} and col {col}: {value}", row=row, col=col, value=value);
            match value {
                '0'..='9' | '.' => (),
                _ => return true,
            }
        }
    }
    false
}
