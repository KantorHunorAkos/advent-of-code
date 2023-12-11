use std::fs::read_to_string;

fn main() {
    let content = read_to_string("../data.txt").unwrap();
    let mut lines = content.lines();
    let times: u64 = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .replace(" ", "")
        .parse()
        .unwrap()
    ;
    let distances: u64 = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .replace(" ", "")
        .parse()
        .unwrap()
    ;
    let mut result: u64 = 1;
    let mut l: u64 = 1;
    while l * (times - l) <= distances { l += 1; }
    result *= times - 2*l + 1;
    println!("{}", l);
    println!("{}", result);
}
