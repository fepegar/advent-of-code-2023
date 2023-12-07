use std::fs::read_to_string;

fn main() {
    println!("PART 1");
    println!("------");
    part_1("example.txt");
    // part_1("input.txt");
    // println!("");
    // println!("PART 2");
    // println!("------");
    // part_2("example.txt");
    // part_2("input.txt");
}


fn part_1(filename: &str) {
    let contents = read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut all_button_durations = Vec::new();
    // all_button_durations.push(get_num_winning_button_durations(7, 9));
    // all_button_durations.push(get_num_winning_button_durations(15, 40));
    // all_button_durations.push(get_num_winning_button_durations(30, 200));
    all_button_durations.push(get_num_winning_button_durations(54, 302));
    all_button_durations.push(get_num_winning_button_durations(94, 1476));
    all_button_durations.push(get_num_winning_button_durations(65, 1029));
    all_button_durations.push(get_num_winning_button_durations(92, 1404));
    let product = all_button_durations.iter().product::<u32>();
    println!("{product}");
}


fn get_num_winning_button_durations(race_duration: u32, record: u32) -> u32 {
    let button_durations = get_winning_button_durations(race_duration, record);
    button_durations.len() as u32
}


fn get_winning_button_durations(race_duration: u32, record: u32) -> Vec<u32> {
    let mut button_durations = Vec::new();
    let mut distance;
    for button_duration in 0..=race_duration {
        distance = get_distance(button_duration, race_duration);
        if distance > record {
            button_durations.push(button_duration);
        }
    }
    button_durations
}


fn get_distance(button_duration: u32, race_duration: u32) -> u32 {
    let moving_time = race_duration - button_duration;
    let speed = button_duration;
    let distance = speed * moving_time;
    distance
}
