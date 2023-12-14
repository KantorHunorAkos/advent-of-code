use std::fs::read_to_string;

fn main() {
    let mut galaxies: Vec<(u64, u64)> = Vec::new();
    let mut x: u64 = 0;
    const OLD: u64 = 1000000;

    let content = read_to_string("../data.txt").unwrap();
    let vertical_size = content.lines().count();
    let horizontal_size = content.lines().next().unwrap().chars().count();
    for line in content.lines() {
        let mut y: u64 = 0;
        for point in line.chars() {
            if point == '#' {
                galaxies.push((x, y));
            }
            y += 1;
        }
        x += 1;
    }
    
    let mut vertical_space_expands: Vec<u64> = Vec::new();
    for x in 0..vertical_size {
        let mut has_galaxy: bool = false;
        for galaxy in &galaxies {
            if x == (galaxy.0 as usize) {
                has_galaxy = true;
                break;
            }
        }
        if !has_galaxy {
            let vlen = vertical_space_expands.len();
            vertical_space_expands.push(((x+vlen) as u64)*OLD);
        }
    }

    let mut horizontal_space_expands: Vec<u64> = Vec::new();
    for y in 0..horizontal_size {
        let mut has_galaxy: bool = false;
        for galaxy in &galaxies {
            if y == (galaxy.1 as usize) {
                has_galaxy = true;
                break;
            }
        }
        if !has_galaxy {
            let hlen = horizontal_space_expands.len();
            horizontal_space_expands.push(((y+hlen) as u64)*OLD);
        }
    }

    for x in vertical_space_expands {
        let _ = galaxies.iter_mut().for_each(|galaxy| if galaxy.0 > x { galaxy.0 += OLD; });
    }
    for y in horizontal_space_expands {
        let _ = galaxies.iter_mut().for_each(|galaxy| if galaxy.1 > y { galaxy.1 += OLD; });
    }


    let mut sum = 0;
    let len = galaxies.len();
    for i in 0..len {
        let mut num = 0;
        for j in i+1..len {
            num += galaxies[i].0.abs_diff(galaxies[j].0) + galaxies[i].1.abs_diff(galaxies[j].1); 
        }
        sum += num;
    }
    println!("{}", sum);
}
