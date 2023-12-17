use std::fs::read_to_string;

fn main() {
    let content = read_to_string("../data.txt").unwrap();
    let mut lines = content.lines();
    
    let first = lines.next().unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    first.chars().for_each(|ch|
        map.push(vec![ch])
    );

    lines.for_each(|l| {
        let mut i: usize = 0;
        l.chars().for_each(|ch| {
            map[i].push(ch);
            i += 1;
        });
    });

    let mut sum = 0u128;
    for line in map {
        let mut pos: u128 = line.len() as u128;
        for i in 0..line.len() {
            if line[i] == '#' {
                pos = (line.len()-i-1) as u128;
                continue;
            }
            if line[i] == 'O' {
                sum += pos;
                pos -= 1;
            }
        }
    }
    println!("Total sum:{}", sum);
}
