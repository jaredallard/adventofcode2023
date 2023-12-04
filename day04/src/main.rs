use std::collections::HashSet;
use std::fs;
use std::io::{self, prelude::*};

fn main() {
    let f = fs::File::open("day04/input.txt").expect("failed to open file");
    let reader = io::BufReader::new(f);

    let mut card_values_sum = 0;

    for l in reader.lines() {
        let line = l.expect("failed to read line");

        let mut spl = line.split(": ");
        let card_num = spl
            .next()
            .expect("failed to get card num")
            .strip_prefix("Card ")
            .expect("failed to remove Card from card...?")
            .to_string()
            .trim()
            .parse::<usize>()
            .expect("failed to parse card num as int");

        let card = spl.next().expect("failed to get rounds from string");

        let mut sections = card.split(" | ");

        let winning_numbers = sections
            .next()
            .expect("failed to read winning numbers")
            .split_ascii_whitespace()
            .map(|s| s.trim().to_string().parse::<usize>().unwrap_or(0))
            .collect::<HashSet<usize>>();
        let numbers = sections
            .next()
            .expect("failed to read card numbers")
            .split_ascii_whitespace()
            .map(|s| s.trim().to_string().parse::<usize>().unwrap_or(0));

        let mut card_value = 0;
        for n in numbers {
            if winning_numbers.contains(&n) {
                if card_value == 0 {
                    card_value = 1
                } else {
                    card_value *= 2
                }
            }
        }

        println!("Card {}: {}", card_num, card_value);

        card_values_sum += card_value
    }

    println!("Problem 1: {}", card_values_sum)
}
