use std::fs::read_to_string;

fn main() {
    let content = read_to_string("../data.txt").unwrap();
    let data = content.lines().next().unwrap();
    let mut sum = 0u128;
    for step in data.split(",") {
        let mut current_hash = 0u16;
        for ch in step.as_bytes() {
            current_hash = ((*ch) as u16 + current_hash) * 17 % 256;
        }
        println!("{}", current_hash);
        sum += current_hash as u128;
    }
    println!("Total sum: {}", sum);
}
