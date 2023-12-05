use std::fs::read_to_string;

fn main() {
    let mut sum = 0;

    for card in read_to_string("../data.txt").unwrap().lines() {
        let mut points = 0;
        let (winning_numbers, scratched_numbers) = card
            .split_once(":")
            .unwrap()
            .1
            .split_once("|")
            .unwrap()
        ;
        let winning_numbers = winning_numbers
            .split(" ")
            .filter_map(|num| num.parse::<u32>().ok())
        ;
        let scratched_numbers = scratched_numbers
            .split(" ")
            .filter_map(|num| num.parse::<u32>().ok())
            .collect::<Vec<u32>>()
        ;
        for wnum in winning_numbers {
            for snum in &scratched_numbers {
                if wnum == *snum {
                    if points == 0 {
                        points = 1;
                    } else {
                        points = points << 1;
                    }
                }
            }   
        }
        sum += points;
    }

    println!("{}", sum);
}
