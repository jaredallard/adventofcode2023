use std::io::{self, prelude::*};
use std::{fs, vec};

// Wrong: 491712
// Wrong: 493642
// Wrong: 518514

// is_special_char returns true if the provided character is not a digit
// or a '.'.
fn is_special_char(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn main() {
    let f = fs::File::open("day03/input.txt").expect("failed to open file");
    let reader = io::BufReader::new(f);

    let mut graph: Vec<Vec<char>> = vec![];

    for l in reader.lines() {
        graph.push(
            l.expect("failed to read line")
                .chars()
                .collect::<Vec<char>>(),
        );
    }

    // Iterate over the graph, finding numbers that have an adjacent
    // symbol next to them.

    let mut sum = 0;
    for y in 0..graph.len() {
        let row = graph.get(y).expect("should succeed");

        let mut numbers: Vec<char> = vec![];
        let mut numbers_has_symbol = false;

        for x in 0..row.len() {
            let cur = row.get(x).expect("failed to get self");

            let mut up = '.';
            let mut up_left = '.';
            let mut up_right = '.';
            let mut down = '.';
            let mut down_left = '.';
            let mut down_right = '.';
            let mut left = '.';
            let mut right = '.';

            if y != 0 {
                let up_row = graph.get(y - 1).expect("failed to get above row");
                up = *up_row.get(x).expect("should get self");

                if x != row.len() - 1 {
                    up_right = *up_row.get(x + 1).expect("failed to get up right ");
                }
                if x != 0 {
                    up_left = *up_row.get(x - 1).expect("failed to get up left");
                }
            }
            if y != graph.len() - 1 {
                let down_row = graph.get(y + 1).expect("failed to get down row");
                down = *down_row.get(x).expect("should get self");

                if x != row.len() - 1 {
                    down_right = *down_row.get(x + 1).expect("failed to get down right");
                }
                if x != 0 {
                    down_left = *down_row.get(x - 1).expect("failed to get down left");
                }
            }

            if x != 0 {
                left = *row.get(x - 1).expect("should get left");
            }
            if x != row.len() - 1 {
                right = *row.get(x + 1).expect("can't get right")
            }

            // Not a digit, and we have numbers, we should save it.
            let is_digit = cur.is_digit(10);

            // If we have a digit, track it.
            if is_digit {
                numbers.push(*cur);

                // has a symbol, then mark we had one.
                if is_special_char(up)
                    || is_special_char(down)
                    || is_special_char(left)
                    || is_special_char(right)
                    || is_special_char(up_left)
                    || is_special_char(up_right)
                    || is_special_char(down_left)
                    || is_special_char(down_right)
                {
                    numbers_has_symbol = true
                }
            }

            if !is_digit || x == row.len() - 1 {
                // Otherwise, we're either ending a numbers vec or we're
                // skipping it.
                if !numbers.is_empty() && numbers_has_symbol {
                    let number = numbers.iter().fold(0, |acc, &num| {
                        acc * 10 + num.to_digit(10).expect("failed to parse char as number")
                    });

                    println!("Commit {:?} -> {} (S{})", numbers, number, sum);
                    sum += number;
                }

                // reset
                numbers = vec![];
                numbers_has_symbol = false
            }
        }
    }

    println!("Problem 1: {}", sum)
}
