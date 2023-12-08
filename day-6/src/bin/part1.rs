use std::fs::read_to_string;

fn main() {
    let content = read_to_string("../data.txt").unwrap();
    let mut lines = content.lines();
    let times: Vec<u32> = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split(" ")
        .filter_map(|num| num.parse::<u32>().ok())
        .collect()
    ;
    let distances: Vec<u32> = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split(" ")
        .filter_map(|num| num.parse::<u32>().ok())
        .collect()
    ;
    let mut result: u32 = 1;
    for i in 0..times.len() {
        let mut l: u32 = 1;
        while l * (times[i] - l) <= distances[i] { l += 1; }
        result *= times[i] - 2*l + 1;
        println!("{}", l);
    }
    println!("{}", result);
}
