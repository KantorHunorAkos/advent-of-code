use std::fs::read_to_string;

fn main() {
    let mut data: Vec<Vec<u8>> = Vec::new();
    for line in read_to_string("../test_data.txt").unwrap().lines() {
        let mut data_line: Vec<u8> = Vec::new();
        for ch in line.chars() {
            data_line.push(ch.to_digit(10).unwrap() as u8);
        }
        data.push(data_line);
    }
    data.iter().for_each(|l| {
        l.iter().for_each(|d| print!("{} ", d));
        println!();
    });

}
