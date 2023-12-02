use std::fs::read_to_string;

fn main() {
    let mut sum = 0;
    for line in read_to_string("../data.txt").unwrap().lines() {
        let game = line.split_once(":");
        let game_id = game.unwrap().0;
        let index = game_id.find(" ").unwrap();
        let game_id = game_id.split_at(index).1.trim().parse::<i32>().unwrap();
        let games = game.unwrap().1.split(";").map(|c| c.trim());
        let mut valid_game: bool = true;
        
        for game in games {
            let cubes = game.split(",").map(|c| c.trim());
            for cube in cubes {
                let index = cube.find(" ").unwrap();
                let (number, color) = cube.split_at(index);
                let number = number.trim().parse::<i32>().unwrap();
                let color = color.trim();
                match color {
                    "red" => {
                        if number > 12 {
                            valid_game = false;
                        }
                    },
                    "green" => {
                        if number > 13 {
                            valid_game = false;
                        }
                    },
                    "blue" => {
                        if number > 14 {
                            valid_game = false;
                        }
                    },
                    _ => println!("{}", color)
                }
            }    
        }

        if valid_game {
            sum += game_id;
        }        
    }
    
    println!("{}", sum);
}
