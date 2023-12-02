use std::fs;
use std::io::{self, prelude::*};

fn main() {
    let f = fs::File::open("day02/input.txt").expect("failed to open file");
    let reader = io::BufReader::new(f);

    let p1_red_max = 12;
    let p1_green_max = 13;
    let p1_blue_max = 14;

    let mut valid_game_num_sum = 0;

    let mut p2 = 0;

    for l in reader.lines() {
        let line = l.expect("failed to read line");

        let mut spl = line.split(":");
        let game_num = spl
            .next()
            .expect("failed to get game num")
            .strip_prefix("Game ")
            .expect("failed to remove Game from game...?")
            .to_string()
            .parse::<i32>()
            .expect("failed to parse game num as int");
        let rounds = spl.next().expect("failed to get rounds from string");

        // rounds format: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        println!("Game {}", game_num);
        let mut all_valid = true;

        let mut largest_red = 0;
        let mut largest_green = 0;
        let mut largest_blue = 0;

        for (round_num, round) in rounds.split(";").enumerate() {
            let mut round_red = 0;
            let mut round_green = 0;
            let mut round_blue = 0;

            for (_, amount) in round.split(",").enumerate() {
                let mut spl = amount.split(" ");
                spl.next(); // toss

                let amount = spl
                    .next()
                    .expect("failed to parse amount of color")
                    .to_string()
                    .parse::<i32>()
                    .expect("failed to parse amount as int");
                let color = spl.next().expect("failed to parse color");

                if color == "red" {
                    if amount > largest_red {
                        largest_red = amount
                    }

                    round_red += amount
                } else if color == "green" {
                    if amount > largest_green {
                        largest_green = amount
                    }

                    round_green += amount
                } else if color == "blue" {
                    if amount > largest_blue {
                        largest_blue = amount
                    }

                    round_blue += amount
                } else {
                    panic!("unknown color {}", color)
                }
            }

            println!(
                " {} => R{},G{},B{}",
                round_num, round_red, round_green, round_blue
            );

            if !(round_red <= p1_red_max
                && round_green <= p1_green_max
                && round_blue <= p1_blue_max)
            {
                println!("Round {} invalid", round_num);
                all_valid = false;
            }
        }
        if all_valid {
            valid_game_num_sum += game_num;
        }
        println!(
            " = Max(R{}, G{}, B{})",
            largest_red, largest_green, largest_blue
        );

        p2 += (largest_red * largest_green) * largest_blue
    }

    println!("");
    println!("Problem 1: {}", valid_game_num_sum);
    println!("Problem 2: {}", p2);
}
