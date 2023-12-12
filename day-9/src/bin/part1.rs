use std::fs::read_to_string;

fn only_zeros(v: &Vec<i32>) -> bool {
    for item in v {
        if *item != 0 { return false; }
    }
    true
}

fn main() {
    let mut sum_last: i32 = 0;
    let mut sum_first: i32 = 0;
    for line in read_to_string("../data.txt").unwrap().lines() {
        let mut history: Vec<i32> = line.split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let mut rows: Vec<Vec<i32>> = Vec::new();
        rows.push(history.clone());
        loop {
            if only_zeros(&history) {
                break;
            }
            let mut descendants: Vec<i32> = Vec::new();
            for i in 0..history.len()-1 {
                descendants.push(history[i+1]-history[i]);
            } 
            history = descendants;
            rows.push(history.clone());
        }
        
        for row_num in (0..rows.len()-1).rev() {
            let last_current_row = rows[row_num][rows[row_num].len()-1];
            let last_below_row = rows[row_num+1][rows[row_num+1].len()-1];

            rows[row_num].push(last_below_row+last_current_row);

            let first_current_row = rows[row_num][0];
            let first_below_row = rows[row_num+1][0];

            let mut tmp = vec![first_current_row-first_below_row];
            tmp.append(&mut rows[row_num]);
            rows[row_num] = tmp;
        }

        sum_last += rows[0].last().unwrap();
        sum_first += rows[0].first().unwrap();
    }
    println!("{}", sum_first);
    println!("{}", sum_last);
}
