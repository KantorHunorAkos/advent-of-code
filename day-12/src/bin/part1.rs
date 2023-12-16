use std::{fs::read_to_string, ops::Index};

fn count_dot(springs: &[u8], numbers: &mut Vec<u8>) -> u128 {
    count(&springs[1..], numbers)
}

fn count_hash(springs: &[u8], numbers: &mut Vec<u8>) -> u128 {
    let number = numbers[0] as usize;
    if springs.len() < number {
        return 0;
    }
    if springs[..number].contains(&b'.') {
        return 0;
    }
    if springs.len() == number {
        if numbers.len() == 1 {
            return 1;
        } else {
            return 0;
        }
    }
    if springs[number] == b'?' || springs[number] == b'.' {
        return count(&springs[number+1..], &mut numbers[1..].to_vec());
    }
    0
}

fn count(springs: &[u8], numbers: &mut Vec<u8>) -> u128 {
    if numbers.is_empty() {
        return !springs.contains(&b'#') as u128;
    }

    if springs.is_empty() {
        return 0;
    }

    let spring = springs.index(0);

    match spring {
        b'.' => count_dot(springs, numbers),
        b'#' => count_hash(springs, numbers),
        b'?' => count_dot(springs, numbers) + count_hash(springs, numbers),
        _ => panic!("Unknown character in springs")
    }
}

fn main() {
    let mut sum: u128 = 0;
    for line in read_to_string("../data.txt").unwrap().lines() {
        let (springs, numbers) = line.split_once(" ").unwrap();
        let springs = springs.as_bytes();
        let mut numbers: Vec<u8> = numbers.split(",").map(|n| n.parse::<u8>().unwrap() ).collect();
        sum += count(springs, &mut numbers);
    }
    println!("Total sum: {}", sum);
}
