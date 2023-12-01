use std::fs::read_to_string;

fn part1(filename: &str) -> i32 {
    let mut sum :i32 = 0;

    for line in read_to_string(filename).unwrap().lines() {
        let mut number = String::from("");
        for ch in line.to_string().chars() {
            if ch.is_digit(10) {
                number.push(ch);
                break;
            }
        }
        
        for ch in line.to_string().chars().rev() {
            if ch.is_digit(10) {
                number.push(ch);
                break;
            }
        }

        sum += number.parse::<i32>().unwrap();
    }

    sum
}

fn main() {
    let sum = part1("../data.txt");

    println!("{}", sum);
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let res = part1("../test_data.txt");
        assert_eq!(142, res);
    }
}
