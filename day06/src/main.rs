use std::fs;
use std::io::{self, prelude::*};

use itertools::Itertools;

fn main() {
    let f = fs::File::open("day06/input.txt").expect("failed to open file");
    let mut lines = io::BufReader::new(f).lines();

    let times = lines
        .next()
        .expect("failed to read line")
        .expect("should be stringable...")
        .strip_prefix("Time:")
        .expect("no Time: prefix")
        .split_ascii_whitespace()
        .map(|s| return s.parse::<usize>().expect("failed to turn into usize"))
        .collect::<Vec<usize>>();
    let distances = lines
        .next()
        .expect("failed to read line")
        .expect("should be stringable...")
        .strip_prefix("Distance:")
        .expect("no Distance: prefix")
        .split_ascii_whitespace()
        .map(|s| return s.parse::<usize>().expect("failed to turn into usize"))
        .collect::<Vec<usize>>();

    let part1 = times
        .iter()
        .map(|&time| {
            // build possible times
            (1..time).map(|i| i * (time - i)).collect::<Vec<usize>>()
        })
        .enumerate()
        .map(|(i, opts)| {
            // find options that are better than stated
            opts.iter().filter(|&o| o > &distances[i]).count()
        })
        .reduce(|a, b| return a * b)
        .expect("numbers should multiply");

    // turn [1, 2, 3] into 123
    let joined_times = times
        .iter()
        .map(|&time| time.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<usize>()
        .expect("failed to parse joined times as usize");
    let joined_distance = distances
        .iter()
        .map(|&distance| distance.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<usize>()
        .expect("failed to parse joined distance as usize");

    let part2 = (0..joined_times)
        .map(|i| (joined_times - i) * i)
        .filter(|&o| o > joined_distance)
        .count();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2); // wrong: 58332
}
