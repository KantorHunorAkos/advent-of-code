use std::fs::read_to_string;

fn vertical_mirror(map: &Vec<&[u8]>) -> u128 {
    let lenght = map[0].len();
    let width = map.len();
    let mut mirror: bool = true;

    for j in 0..width {
        if map[j][0] != map[j][1] {
            mirror = false;
            break;
        }
    }
    if mirror {
        return 1;
    }


    for i in 1..lenght-1 {
        mirror = true;
        let lesser = if i < lenght/2 { i } else { lenght-2-i };
        for j in 0..=lesser {
            for k in 0..width {
                if map[k][i-j] != map[k][i+j+1] {
                    mirror = false;
                    break;
                }
            }
            if !mirror {
                break;
            }
        }
        if mirror {
            return (i+1) as u128;
        }
    }
    0u128
}

fn horizontal_mirror(map: &Vec<&[u8]>) -> u128 {
    let lenght = map[0].len();
    let width = map.len();
    let mut mirror: bool = true;

    for j in 0..lenght {
        if map[0][j] != map[1][j] {
            mirror = false;
            break;
        }
    }
    if mirror {
        return 1;
    }

    for i in 1..width-1 {
        mirror = true;
        let lesser = if i < width/2 { i } else { width-2-i };
        for j in 0..=lesser {
            for k in 0..lenght {
                if map[i-j][k] != map[i+j+1][k] {
                    mirror = false;
                    break;
                }
            }
            if !mirror {
                break;
            }
        }
        if mirror {
            return (i+1) as u128;
        }
    }

    0u128
}


fn main() {
    let mut map: Vec<&[u8]> = Vec::new();
    let mut sum = 0u128;
    let maps = read_to_string("../data.txt").unwrap();
    for line in maps.lines() {
        if line.is_empty() {
            map.iter().for_each(|line| {line.iter().for_each(|c| print!("{}", *c as char)); println!();});
            let vm = vertical_mirror(&map);
            let hm = horizontal_mirror(&map);
            println!("v{} h{}", vm, hm);
            println!();
            sum += if vm != 0 { vm } else { 100*hm };
            map.clear();
            continue;
        }
        map.push(line.as_bytes());
    }
    sum += vertical_mirror(&map) + 100*horizontal_mirror(&map);
    println!("TOTAL SUM: {}", sum);
}
