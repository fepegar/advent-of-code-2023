const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;


fn main() {
    part_1();
}


fn part_1() {
    println!("PART 1");
    println!("------");
    print_sum("example.txt");
    print_sum("input.txt");
}


fn print_sum(filename: &str) {
    let mut sum = 0;
    let contents = std::fs::read_to_string(filename).expect("Error reading file");
    let lines = contents.split("\n");
    for line in lines {
        let game_id = get_game_id(line);
        let game_sets = get_game_sets(line);
        let game = Game {
            id: game_id,
            sets: game_sets,
        };
        let is_valid = is_game_valid(&game);
        if is_valid {
            sum += game.id;
        }
    }
    println!("{filename}: {sum}");
}


fn get_game_id(game_string: &str) -> u32 {
    let split_game_string = game_string.split(": ");
    let mut game_id = 0;
    for (i, word) in split_game_string.enumerate() {
        if i == 0 {
            let split_game_id = word.split(" ");
            for (j, word) in split_game_id.enumerate() {
                if j == 1 {
                    game_id = word.parse::<u32>().unwrap();
                }
            }
        }
    }
    game_id
}


fn build_cubes_set(string: &str) -> CubesSet {
    // For example, "3 blue, 4 red" or "2 red, 1 green, 5 blue" or "1 green"
    let count_strings = string.split(", ");
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for count_string in count_strings {
        let count_and_color = count_string.split(" ");
        let mut count = 0;
        let mut color = "";
        for word in count_and_color {
            if count == 0 {
                count = word.parse::<u32>().unwrap();
            } else {
                color = word;
            }
        }
        if color == "red" {
            red = count;
        } else if color == "green" {
            green = count;
        } else if color == "blue" {
            blue = count;
        }
    }
    CubesSet {
        red: red,
        green: green,
        blue: blue,
    }
}


fn get_game_sets(game_string: &str) -> Vec<CubesSet> {
    // For example, "Game 8: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    // Get sets
    let split_game_string = game_string.split(": ");
    let mut sets = Vec::new();
    for (i, word) in split_game_string.enumerate() {
        if i == 1 {
            let split_sets = word.split("; ");
            for set_string in split_sets {
                let cubes = build_cubes_set(set_string);
                sets.push(cubes);
            }
        }
    }
    sets
}


struct CubesSet {
    red: u32,
    green: u32,
    blue: u32,
}


struct Game {
    id: u32,
    sets: Vec<CubesSet>,
}


fn is_game_valid(game: &Game) -> bool {
    for set in &game.sets {
        if set.red > MAX_RED || set.green > MAX_GREEN || set.blue > MAX_BLUE {
            return false;
        }
    }
    true
}
