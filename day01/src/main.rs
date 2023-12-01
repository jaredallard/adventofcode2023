use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io::{self, prelude::*};

fn main() {
    let words_to_numbers: HashMap<&str, i16> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let f = fs::File::open("day01/input.txt").expect("failed to open file");
    let reader = io::BufReader::new(f);

    let mut sum: i32 = 0;

    for l in reader.lines() {
        let line = l.expect("faled to read line");

        // numbers stores all of the numbers found in the line.
        let mut numbers: Vec<i16> = Vec::new();

        // partials stores characters that partially match a key in
        // words_to_numbers. This should be reset any time a valid word
        // or digit is found.
        let mut partials: VecDeque<char> = VecDeque::new();

        for c in line.chars() {
            // digit, add it to the numbers vec.
            if c.is_digit(10) {
                // reset the partial matcher.
                partials.clear();

                numbers.push(c.to_digit(10).unwrap() as i16);
                continue;
            }

            // If not alphabetic char, skip...
            if !c.is_alphabetic() {
                continue;
            }

            // add the character to the partial match vec.
            partials.push_back(c);

            // see if we have a direct match with word -> number.
            let word: String = partials.iter().collect();
            let word_key = word.as_str();

            // Check if the current_word is equal to a key in the map.
            // If so, use that number and finish.
            if words_to_numbers.contains_key(word_key) {
                let number = words_to_numbers[word_key];
                numbers.push(number);
                continue;
            }

            // Iterate over the partials (characters) and see if we have
            // a match (starts with) with all of the characters. If we
            // don't, remove the first character and try again until we
            // have nothing left, or we have a new match.
            let keys = words_to_numbers.keys();
            while !partials.is_empty() {
                let word: String = partials.iter().collect();
                let word_key = word.as_str();

                let mut matched = false;
                for k in keys.clone() {
                    if k.starts_with(word_key) {
                        matched = true;
                        break;
                    }
                }
                if matched {
                    break;
                }

                // try again, but this time remove the first character
                // from the partials vector.
                partials.pop_front();
            }
        }

        // append both numbers together.
        let first = numbers.first().expect("failed to get first number");
        let last = numbers.last().expect("failed to get last number");

        let joined = (first * 10) + last;

        println!("{}: {:?} = {}", line, numbers, joined);

        sum += joined as i32
    }

    println!("Problem 2: {}", sum)
}
