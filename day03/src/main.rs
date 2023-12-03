use std::io::{self, prelude::*};
use std::{fs, vec};

// Wrong: 491712
// Wrong: 493642
// Wrong: 518514

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Symbol {
    // pos is a representation of the location of the symbol as it would
    // appear on a 2d array ()
    pos: Vec<usize>,
}

// is_special_char returns true if the provided character is not a digit
// or a '.'.
fn is_special_char(c: &char) -> bool {
    !c.is_digit(10) && *c != '.'
}

// process_edges returns all symbols found within the provided edges.
fn has_special_char_edge(edges: Vec<Option<&char>>) -> bool {
    for e in edges.iter() {
        let c = e.unwrap_or(&'.');
        if is_special_char(c) {
            return true;
        }
    }

    // didn't find one, v sad.
    return false;
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

        let mut numbers: Vec<usize> = vec![];
        let mut numbers_has_symbol = false;

        // The horrible == 0 checks are because we can't look for
        // -1. I'm sorry. I'm so bad at this.
        for x in 0..row.len() {
            let cur = row.get(x).expect("failed to get self");

            let default_row: &Vec<char> = &vec![];
            let up_row = graph
                .get(if y == 0 { graph.len() } else { y - 1 })
                .unwrap_or(default_row);
            let down_row = graph.get(y + 1).unwrap_or(default_row);

            let edges = vec![
                up_row.get(if x == 0 { up_row.len() } else { x - 1 }), // up left
                up_row.get(x),                                         // up
                up_row.get(x + 1),                                     // up right
                row.get(x + 1),                                        // right
                down_row.get(x + 1),                                   // down right
                down_row.get(x),                                       // down
                down_row.get(if x == 0 { down_row.len() } else { x - 1 }), // down left
                row.get(if x == 0 { row.len() } else { x - 1 }),       // left
            ];

            // Not a digit, and we have numbers, we should save it.
            let is_digit = cur.is_digit(10);

            // If we have a digit, track it.
            if is_digit {
                numbers.push(
                    cur.to_digit(10)
                        .expect("failed to parse number as a number") as usize,
                );

                // Already found one, don't need to process again.
                if !numbers_has_symbol {
                    if has_special_char_edge(edges) {
                        numbers_has_symbol = true;
                    }
                }
            }

            // If we're not a digit, or if we're at the end of the row,
            // close out the current number (if there is one w/ a
            // symbol) or toss and reset.
            if !is_digit || x == row.len() - 1 {
                // Otherwise, we're either ending a numbers vec or we're
                // skipping it.
                if !numbers.is_empty() && numbers_has_symbol {
                    let number = numbers.iter().fold(0, |acc, &num| acc * 10 + num);

                    println!("Commit {:?} -> {} (S{})", numbers, number, sum);
                    sum += number;
                }

                // find a new number.
                numbers = vec![];
                numbers_has_symbol = false;

                continue;
            }
        }
    }

    // Correct: 539433
    println!("Problem 1: {}", sum)
}
