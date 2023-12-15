use std::fs::read_to_string;

fn is_valid(springs: &str, numbers: Vec<u8>) -> bool {
    let springs = springs.split(".").filter(|s| !s.is_empty() );
    let mut numbers = numbers.clone();
    numbers.reverse();
    for spring in springs {
        if let Some(number) = numbers.pop() {
            println!("{}:{}", number, spring);
            if number != spring.len() as u8 {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn main() {
    let mut sum = 0;
    for line in read_to_string("../test_data.txt").unwrap().lines() {
        let (springs, numbers) = line.split_once(" ").unwrap();
        let numbers: Vec<u8> = numbers.split(",").map(|n| n.parse::<u8>().unwrap() ).collect();

        println!("{}", is_valid(springs, numbers));
    }
    println!("{}", sum);
}
