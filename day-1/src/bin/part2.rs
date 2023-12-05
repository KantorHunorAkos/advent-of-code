use std::fs::read_to_string;

fn main() {
    let mut sum :i32 = 0;
    let digit_str = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for line in read_to_string("../data.txt").unwrap().lines() {
        let mut digits : Vec<(usize, &str)> = vec![]; 

        for digit in digit_str {
            let mut d : Vec<(usize, &str)> = line
                .match_indices(digit)
                .collect::<Vec<(usize, &str)>>()
            ;

            digits.append(&mut d);
        }

        for i in 0..digits.len() {
            digits[i].1 = match digits[i].1 {
                "zero" => "0",
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                _ => "" 
            }
        }

        digits
            .append(&mut line.match_indices(char::is_numeric)
            .collect::<Vec<(usize,&str)>>())
        ;

        digits.sort();

        for d in &digits {
            println!("({}, {})", d.0, d.1);
        }
        let first : i32 = digits.first().unwrap().1.parse().unwrap();
        let last : i32 = digits.last().unwrap().1.parse().unwrap();

        println!("{}{}", first, last);
        sum += first*10 + last;
    }


    println!("{}", sum);
}
