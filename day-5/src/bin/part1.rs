use std::{fs::read_to_string, str::Lines};

struct DataLine {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

fn extract_data(lines: &mut Lines) -> Vec<DataLine>
{
    let _ = lines.next();
    let mut data_lines: Vec<DataLine> = vec![];
    loop {
        let line = lines.next();
        if line.is_none() { 
            break;
        }
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let mut data_line = line.split(" ").filter_map(|x| x.parse::<u64>().ok());
        data_lines.push(DataLine{
            destination_range_start: data_line.next().unwrap(),
            source_range_start: data_line.next().unwrap(),
            range_length: data_line.next().unwrap()
        });   
    }
    data_lines
}

fn from_to(from: u64, map: &Vec<DataLine>) -> u64
{
    let mut to: u64 = from;
    for item in map {
        if from >= item.source_range_start &&
            from < item.source_range_start + item.range_length {
            to = item.destination_range_start + from - item.source_range_start;
        }
    }
    to
}

fn main() {
    let content = read_to_string("../data.txt").unwrap();
    let mut lines = content.lines();

    let seeds = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split(" ")
        .filter_map(|seed| seed.parse::<u64>().ok())
    ;
    lines.next();

    let seed_to_soil = extract_data(&mut lines);
    let soil_to_fertilizer = extract_data(&mut lines);
    let fertilizer_to_water = extract_data(&mut lines);
    let water_to_light = extract_data(&mut lines);
    let light_to_temperature = extract_data(&mut lines);
    let temperature_to_humidity = extract_data(&mut lines);
    let humidity_to_location = extract_data(&mut lines);
    
    let mut solution: u64 = u64::MAX;
    for seed in seeds {
        let soil = from_to(seed, &seed_to_soil);
        let fertilizer = from_to(soil, &soil_to_fertilizer);
        let water = from_to(fertilizer, &fertilizer_to_water);
        let light = from_to(water, &water_to_light);
        let temperature = from_to(light, &light_to_temperature);
        let humidity = from_to(temperature, &temperature_to_humidity);
        let location = from_to(humidity, &humidity_to_location);
        if solution > location {
            solution = location;
        }
    }

    println!("{}", solution);
}
