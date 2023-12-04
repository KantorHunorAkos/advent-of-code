use std::fs::read_to_string;

fn main() {
    let mut input: Vec<char> = vec![];
    let mut line_length: usize = 0;
    for line in read_to_string("../data.txt").unwrap().lines() {
        let l = vec![".", line, "."].concat();
        input.append(&mut l.chars().collect::<Vec<char>>());
        line_length = l.len();
    }

    let mut sum = 0;
    let mut num_len : usize = 0;
    for i in 0..input.len() {
        let mut has_adjacent_symbol: bool = false;
        if input[i].is_digit(10) {
            num_len += 1;
        } else if num_len > 0 {
            if i > line_length {
                for j in 0..num_len+2 {
                    if has_adjacent_symbol { break; }
                    has_adjacent_symbol = match input[i-line_length-j] {
                        '*' | '+' | '$' | '#' | '@' | '%' | '/' | '=' | '-' | '&' => true,
                        _ => false
                    }
                }
            }
            for j in 0..num_len+2 {
                if has_adjacent_symbol { break; }
                has_adjacent_symbol = match input[i-j] {
                    '*' | '+' | '$' | '#' | '@' | '%' | '/' | '=' | '-' | '&' => true,
                    _ => false
                }
            }
            if i + line_length < input.len() {
                for j in 0..num_len+2 {
                    if has_adjacent_symbol { break; }
                    has_adjacent_symbol = match input[i+line_length-j] {
                        '*' | '+' | '$' | '#' | '@' | '%' | '/' | '=' | '-' | '&' => true,
                        _ => false
                    }
                }
            }
            if has_adjacent_symbol {
                let mut num = String::from("");
                for j in 0..num_len {
                    num.push(input[i+j-num_len]);
                }
                sum += num.parse::<u32>().unwrap();
            }
            num_len = 0;
        }
    }

    println!("{}", sum);
}
