use std::{fs::read_to_string, collections::HashMap};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(PartialEq, Eq, Hash)]
struct Point(usize, usize, Direction);

fn go_light_go(
    map: &Vec<String>,
    x: usize,
    y: usize,
    dir: Direction,
    visited: &mut HashMap<(usize, usize, Direction), u128>
) {
    let current = map[x].as_bytes()[y];
    //print!("{}", current as char);
    visited.entry((x, y, dir)).and_modify(|count| *count += 1).or_insert(1);
    if visited.get(&(x, y, dir)).unwrap() > &1 { return; } 
    match current {
        b'|' => {
            if x > 0 {
                go_light_go(map, x-1, y, Direction::UP, visited);
            }
            if x+1 < map.len() && dir != Direction::UP {
                go_light_go(map, x+1, y, Direction::DOWN, visited);
            }
        },
        b'-' => {
            if y > 0 {
                go_light_go(map, x, y-1, Direction::LEFT, visited);
            }
            if y+1 < map[x].len() && dir != Direction::LEFT {
                go_light_go(map, x, y+1, Direction::RIGHT, visited);
            }
        },
        b'\\' => match dir {
            Direction::LEFT => if x > 0 {
                go_light_go(map, x-1, y, Direction::UP, visited);
            },
            Direction::RIGHT => if x+1 < map.len() {
                go_light_go(map, x+1, y, Direction::DOWN, visited);
            },
            Direction::UP => if y > 0 {
                go_light_go(map, x, y-1, Direction::LEFT, visited);
            },
            Direction::DOWN => if y+1 < map[x].len() {
                go_light_go(map, x, y+1, Direction::RIGHT, visited);
            },
        },
        b'/' => match dir {
            Direction::LEFT => if x+1 < map.len() {
                go_light_go(map, x+1, y, Direction::DOWN, visited);
            },
            Direction::RIGHT => if x > 0 {
                go_light_go(map, x-1, y, Direction::UP, visited);
            },
            Direction::UP => if y+1 < map[x].len() {
                go_light_go(map, x, y+1, Direction::RIGHT, visited);
            },
            Direction::DOWN => if y > 0 {
                go_light_go(map, x, y-1, Direction::LEFT, visited);
            },
        },
        b'.' =>  match dir {
            Direction::DOWN => if x+1 < map.len() {
                go_light_go(map, x+1, y, Direction::DOWN, visited);
            },
            Direction::UP => if x > 0 {
                go_light_go(map, x-1, y, Direction::UP, visited);
            },
            Direction::RIGHT => if y+1 < map[x].len() {
                go_light_go(map, x, y+1, Direction::RIGHT, visited);
            },
            Direction::LEFT => if y > 0 {
                go_light_go(map, x, y-1, Direction::LEFT, visited);
            },
        },
        _ => {}
    }
}

fn main() {
    let content = read_to_string("../data.txt").unwrap();
    let lines = content.lines();
    let mut max_energized = 0u128;
    let line_count = lines.clone().count();
    let line_len = lines.clone().last().unwrap().len();
    let mut energized = 0u128;
    let mut map: Vec<String> = Vec::new();
    let mut visited: HashMap<(usize, usize, Direction), u128> = HashMap::new();

    for i in 0..line_len {
        energized = 0u128;
        map.clear();
        visited.clear();

        lines.clone().for_each(|line| map.push(line.to_string()));
        go_light_go(&map, 0, i, Direction::DOWN, &mut visited);
        visited.keys().for_each(|k| map[k.0].replace_range(k.1..=k.1, "#"));
        map.iter().for_each(|s| s.chars().for_each(|ch| if ch == '#' { energized += 1 }));
        max_energized = max_energized.max(energized);
    }

    for i in 0..line_len {
        energized = 0u128;
        map.clear();
        visited.clear();

        lines.clone().for_each(|line| map.push(line.to_string()));
        go_light_go(&map, line_count-1, i, Direction::UP, &mut visited);
        visited.keys().for_each(|k| map[k.0].replace_range(k.1..=k.1, "#"));
        map.iter().for_each(|s| s.chars().for_each(|ch| if ch == '#' { energized += 1 }));
        max_energized = max_energized.max(energized);
    }

    for i in 0..line_count {
        energized = 0u128;
        map.clear();
        visited.clear();

        lines.clone().for_each(|line| map.push(line.to_string()));
        go_light_go(&map, i, line_len-1, Direction::LEFT, &mut visited);
        visited.keys().for_each(|k| map[k.0].replace_range(k.1..=k.1, "#"));
        map.iter().for_each(|s| s.chars().for_each(|ch| if ch == '#' { energized += 1 }));
        max_energized = max_energized.max(energized);
    }

    for i in 0..line_count {
        energized = 0u128;
        map.clear();
        visited.clear();

        lines.clone().for_each(|line| map.push(line.to_string()));
        go_light_go(&map, i, 0, Direction::RIGHT, &mut visited);
        visited.keys().for_each(|k| map[k.0].replace_range(k.1..=k.1, "#"));
        map.iter().for_each(|s| s.chars().for_each(|ch| if ch == '#' { energized += 1 }));
        max_energized = max_energized.max(energized);
    }

    println!("Maximum overdrive: {}", max_energized);
}
