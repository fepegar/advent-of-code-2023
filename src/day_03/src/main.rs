use std::cmp;
use std::string;
use std::collections::HashMap;
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


fn part_1(filename: &str) {
    let contents = read_file(filename);
    let schematic = get_schematic(contents);
    let numbers = get_numbers_from_schematic(&schematic);
    let mut sum = 0;
    for number in numbers {
        if is_part(&number, &schematic) {
            sum += number.value;
        }
    }
    println!("{filename}: {sum}");
}


fn read_file(filename: &str) -> string::String {
    let contents = read_to_string(filename).expect("Error reading file");
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
        let col_fin = cmp::min(number.col_fin, num_cols - 1);
        for col in col_ini..=col_fin {
            let value = schematic[row as usize][col as usize];
            match value {
                '0'..='9' | '.' => (),
                _ => {
                    return true;
                },
            }
        }
    }
    false
}


fn part_2(filename: &str) {
    let contents = read_file(filename);
    let schematic = get_schematic(contents);
    let numbers = get_numbers_from_schematic(&schematic);
    let numbers_and_gears = get_numbers_and_gears(&numbers, &schematic);
    let gears_dict = get_gears_dict(&numbers_and_gears);
    let mut sum = 0;
    // Iterate over the gears_dict
    for (_coords, values) in gears_dict.iter() {
        let num_numbers = values.len();
        match num_numbers {
            0 => (),
            1 => (),
            2 => {
                let first_value = values[0];
                let second_value = values[1];
                sum += first_value * second_value;
            },
            _ => (),
        }
    }
    println!("{filename}: {sum}");
}


fn get_numbers_and_gears(numbers: &Vec<Number>, schematic: &Vec<Vec<char>>) -> Vec<NumberAndGear> {
    let mut numbers_and_gears: Vec<NumberAndGear> = Vec::new();
    for number in numbers {
        let num_rows = schematic.len() as i32;
        let num_cols = schematic[0].len() as i32;
        let row_ini = cmp::max(number.row - 1, 0);
        let row_fin = cmp::min(number.row + 1, num_rows - 1);
        for row in row_ini..=row_fin {
            let col_ini = cmp::max(number.col_ini - 1, 0);
            let col_fin = cmp::min(number.col_fin, num_cols - 1);
            for col in col_ini..=col_fin {
                let value = schematic[row as usize][col as usize];
                match value {
                    '*' => {
                        let number_and_gear = NumberAndGear {
                            value: number.value,
                            gear_row: row,
                            gear_col: col,
                        };
                        numbers_and_gears.push(number_and_gear);
                    },
                    _ => (),
                }
            }
        }
    }
    numbers_and_gears
}


struct NumberAndGear {
    value: u32,
    gear_row: i32,
    gear_col: i32,
}


fn get_gears_dict(numbers_and_gears: &Vec<NumberAndGear>) -> HashMap<(i32, i32), Vec<u32>> {
    let mut gears_dict = HashMap::new();
    for number_and_gear in numbers_and_gears {
        let key = (number_and_gear.gear_row, number_and_gear.gear_col);
        if !gears_dict.contains_key(&key) {
            let empty_list: Vec<u32> = Vec::new();
            gears_dict.insert(key, empty_list);
        }
        let item = number_and_gear.value;
        // Push item to vector in the value
        let value = gears_dict.get_mut(&key).unwrap();
        value.push(item);
    }
    gears_dict
}
