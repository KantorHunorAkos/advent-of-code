use std::fs::read_to_string;


fn starting_position(map: &Vec<Vec<u8>>) -> Result<(usize, usize), ()> {
    for i in 0..map.len() {
        for j in 0..map.len() {
            if map[i][j] == 'S' as u8 {
                return Ok((i, j));
            }
        }
    }
    Err(())
}

fn go(map: &mut Vec<Vec<u8>>, x: usize, y: usize, steps: &mut u32) {
    match map[x][y] {
        b'-' => {
            map[x][y] = b'.';
            *steps += 1;
            go(map, x, y-1, steps);
            go(map, x, y+1, steps);
        },
        b'|' => {
            map[x][y] = b'.';
            *steps += 1;
            go(map, x-1, y, steps);
            go(map, x+1, y, steps);
        },
        b'J' => {
            map[x][y] = b'.';
            *steps += 1;
            go(map, x, y-1, steps);
            go(map, x-1, y, steps);
        },
        b'L' => {
            map[x][y] = b'.';
            *steps += 1;
            go(map, x-1, y, steps);
            go(map, x, y+1, steps);
        },
        b'F' => {
            map[x][y] = b'.';
            *steps += 1;
            go(map, x+1, y, steps);
            go(map, x, y+1, steps);
        },
        b'7' => {
            map[x][y] = b'.';
            *steps += 1;
            go(map, x+1, y, steps);
            go(map, x, y-1, steps);
        },
        _ => map[x][y] = b'.'
    };
}

fn main() {
    let mut map: Vec<Vec<u8>> = Vec::new();
    let _ = read_to_string("../data.txt").unwrap()
        .lines()
        .for_each(|line| {
            let mut l = vec![b'.'];
            l.append(&mut line.as_bytes().to_vec());
            l.push(b'.');
            map.push(l);
        });
    
    let mut m: Vec<Vec<u8>> = vec![vec![b'.'; map[0].len()]];
    m.append(&mut map);
    m.push(vec![b'.'; m[0].len()]);
    map = m;

    map.iter().for_each(|l| {
        l.iter().for_each(|x| print!("{} ", *x as char));
        println!();
    });

    let (x, y) = match starting_position(&map) {
        Ok(pos) => pos,
        Err(_) => panic!("'S' not found")
    };
    println!("{}:{}", x, y);
    map[x][y] = b'.';

    let mut steps = 0;
    if map[x][y+1] == b'-' {
        go(&mut map, x, y+1, &mut steps);
    } else if map[x+1][y] == b'|' {
        go(&mut map, x+1, y, &mut steps);
    } else if map[x][y-1] == b'-' {
        go(&mut map, x, y-1, &mut steps);
    } else if map[x-1][y] == b'|' {
        go(&mut map, x-1, y, &mut steps);
    }
    map.iter().for_each(|l| {
        l.iter().for_each(|x| print!("{} ", *x as char));
        println!();
    });

    println!("{}", (steps+1) /2);
}
