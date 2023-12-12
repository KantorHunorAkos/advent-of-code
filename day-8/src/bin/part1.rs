use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let content = read_to_string("../data.txt").unwrap();
    let mut lines = content.lines();
    let instructions = lines.next().unwrap().as_bytes();

    let map: HashMap<&str, (&str, &str)> = lines.skip(1)
        .map(|l| l.split_once(" = ").unwrap())
        .map(|(t0, t1)| { 
            let mut chars = t1.chars();
            chars.next();
            chars.next_back();
            let lr = chars.as_str().split_once(", ").unwrap();
            (t0, lr)
        })
        .collect();
    
    let mut steps: u32 = 0;
    let mut position = "AAA";
    while position != "ZZZ" {
        println!("{}", position);
        match instructions[(steps as usize) % instructions.len()] {
            76 => position = map.get(position).unwrap().0,
            82 => position = map.get(position).unwrap().1,
            _ => panic!("GO UP!")
        };
        steps += 1;
    }
    println!("{}", steps);
}
