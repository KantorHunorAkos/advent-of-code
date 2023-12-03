use std::fs::read_to_string;

fn main() {
    let mut sum = 0;
    for line in read_to_string("../data.txt").unwrap().lines() {
        let game = line.split_once(":");
        let games = game.unwrap().1.split(";").map(|c| c.trim());
        let mut max_red: i32 = -1;
        let mut max_green: i32 = -1;
        let mut max_blue: i32 = -1;

        for game in games {
            let cubes = game.split(",").map(|c| c.trim());
            for cube in cubes {
                let index = cube.find(" ").unwrap();
                let (number, color) = cube.split_at(index);
                let number = number.trim().parse::<i32>().unwrap();
                let color = color.trim();
                match color {
                    "red" => {
                        if number > max_red {
                            max_red = number;
                        }
                    },
                    "green" => {
                        if number > max_green {
                            max_green = number;
                        }
                    },
                    "blue" => {
                        if number > max_blue {
                            max_blue = number;
                        }
                    },
                    _ => println!("{}", color)
                }
            }
        }

        sum += max_red * max_green * max_blue;
    }
    
    println!("{}", sum);
}
