use std::{fs::read_to_string, str::Lines};

struct DataLine {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32,
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
        let mut data_line = line.split(" ").filter_map(|x| x.parse::<u32>().ok());
        data_lines.push(DataLine{
            destination_range_start: data_line.next().unwrap(),
            source_range_start: data_line.next().unwrap(),
            range_length: data_line.next().unwrap()
        });   
    }
    data_lines
}

fn main() {
    let content = read_to_string("../test_data.txt").unwrap();
    let mut lines = content.lines();

    let mut seeds = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split(" ")
        .filter_map(|seed| seed.parse::<u32>().ok())
    ;
    lines.next();

    let seed_to_soil = extract_data(&mut lines);
    let soil_to_fertilizer = extract_data(&mut lines);
    let fertilizer_to_water = extract_data(&mut lines);
    let water_to_light = extract_data(&mut lines);
    let light_to_temperature = extract_data(&mut lines);
    let temperature_to_humidity = extract_data(&mut lines);
    let humidity_to_location = extract_data(&mut lines);
}
