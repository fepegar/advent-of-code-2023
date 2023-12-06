use std::fs::read_to_string;

fn main() {
    println!("PART 1");
    println!("------");
    part_1("example.txt");
    part_1("input.txt");
}


fn part_1(filename: &str) {
    let contents = read_to_string(filename)
        .expect("Something went wrong reading the file");
    let split = contents.split("\n\n").collect::<Vec<&str>>();
    let seeds_line = split[0];
    let seeds_string = seeds_line
        .split(": ")
        .collect::<Vec<&str>>()[1];
    let seeds = seeds_string
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let mut maps = Vec::new();
    for map_lines in split[1..].iter() {
        let map = Map::from_lines(map_lines);
        maps.push(map);
    }
    let almanac = Almanac { maps };

    let mapped_seeds = seeds.iter().map(|x| almanac.map(*x)).collect::<Vec<u32>>();
    let min_mapped_seed = mapped_seeds.iter().min().unwrap();
    println!("{filename}: {min_mapped_seed}");
}

#[derive(Debug)]
struct Almanac {
    maps: Vec<Map>,
}

impl Almanac {
    fn map(&self, seed: u32) -> u32 {
        let mut mapped = seed;
        for map in self.maps.iter() {
            mapped = map.map(mapped);
    }
        mapped
    }
}

#[derive(Debug, Clone)]
struct Map {
    _source: String,
    _destination: String,
    ranges: Vec<Ranges>,
}

impl Map {
    fn from_lines(lines: &str) -> Map {
        let mut all_ranges = Vec::new();
        let mut source = String::new();
        let mut destination = String::new();
        for (i, line) in lines.lines().enumerate() {
            if i == 0 {
                let name = line.split_whitespace().collect::<Vec<&str>>()[0].to_string();
                (source, destination) = {
                    let mut split = name.split("-");
                    let source = split.next().unwrap().to_string();
                    let _ = split.next();
                    let destination = split.next().unwrap().to_string();
                    (source, destination)
                };
                continue;
            }
            let ranges = Ranges::from_line(line);
            all_ranges.push(ranges);
        }
        Map {
            _source: source,
            _destination: destination,
            ranges: all_ranges,
        }
    }

    fn map(&self, value: u32) -> u32 {
        let mut mapped = value;
        for range in self.ranges.iter() {
            if range.in_range(mapped) {
                mapped = range.map(mapped);
                break;
            }
        }
        mapped
    }
}


#[derive(Debug, Clone)]
struct Ranges {
    dest_range_start: u32,
    source_range_start: u32,
    range_length: u32,
}

impl Ranges {
    fn from_line(line: &str) -> Ranges {
        let mut split = line.split_whitespace();
        let dest_range_start = split.next().unwrap();
        let source_range_start = split.next().unwrap();
        let range_length = split.next().unwrap();
        Ranges {
            dest_range_start: dest_range_start.parse::<u32>().unwrap(),
            source_range_start: source_range_start.parse::<u32>().unwrap(),
            range_length: range_length.parse::<u32>().unwrap(),
        }
    }

    fn in_range(&self, value: u32) -> bool {
        let result = value >= self.source_range_start && value < self.source_range_start + self.range_length;
        result
    }

    fn map(&self, value: u32) -> u32 {
        value - self.source_range_start + self.dest_range_start
    }
}
