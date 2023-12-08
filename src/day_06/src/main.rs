fn main() {
    println!("PART 1");
    println!("------");
    part_1();
    println!("");
    println!("PART 2");
    println!("------");
    part_2();
}


fn part_1() {
    let mut all_button_durations = Vec::new();
    let mut product;
    all_button_durations.push(get_num_winning_button_durations(7, 9));
    all_button_durations.push(get_num_winning_button_durations(15, 40));
    all_button_durations.push(get_num_winning_button_durations(30, 200));
    product = all_button_durations.iter().product::<u64>();
    println!("example.txt: {product}");

    all_button_durations = Vec::new();
    all_button_durations.push(get_num_winning_button_durations(54, 302));
    all_button_durations.push(get_num_winning_button_durations(94, 1476));
    all_button_durations.push(get_num_winning_button_durations(65, 1029));
    all_button_durations.push(get_num_winning_button_durations(92, 1404));
    product = all_button_durations.iter().product::<u64>();
    println!("input.txt: {product}");
}


fn get_num_winning_button_durations(race_duration: u64, record: u64) -> u64 {
    let button_durations = get_winning_button_durations(race_duration, record);
    button_durations.len() as u64
}


fn get_winning_button_durations(race_duration: u64, record: u64) -> Vec<u64> {
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


fn get_distance(button_duration: u64, race_duration: u64) -> u64 {
    let moving_time = race_duration - button_duration;
    let speed = button_duration;
    let distance = speed * moving_time;
    distance
}

fn part_2() {
    let mut product;

    let mut all_button_durations = Vec::new();
    all_button_durations.push(get_num_winning_button_durations(71530, 940200));
    product = all_button_durations.iter().product::<u64>();
    println!("example.txt: {product}");

    all_button_durations = Vec::new();
    all_button_durations.push(get_num_winning_button_durations(54946592, 302147610291404));
    product = all_button_durations.iter().product::<u64>();
    println!("input.txt: {product}");
}
