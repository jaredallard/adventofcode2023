use itertools::Itertools;
use std::io::{self, prelude::*};
use std::{fs, vec};

// Wrong: 491712
// Wrong: 493642
// Wrong: 518514

// Wrong (p2): 52133154

#[derive(Debug, Eq, PartialEq)]
enum Entry {
    Empty,
    Symbol(char),
    NumberPartial(usize),
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

fn part1(graph: Vec<Vec<char>>) -> usize {
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

                    //println!("Commit {:?} -> {} (S{})", numbers, number, sum);
                    sum += number;
                }

                // find a new number.
                numbers = vec![];
                numbers_has_symbol = false;

                continue;
            }
        }
    }

    return sum;
}

fn part2(graph: Vec<Vec<Entry>>) -> usize {
    let graph_range = 0..graph.len();
    graph_range
        .clone()
        .cartesian_product(graph_range)
        .filter(|(y, x)| matches!(graph[*y][*x], Entry::Symbol('*')))
        .map(adjacent_part_nums(&graph))
        .filter_map(|nums| {
            if nums.len() == 2 {
                Some(nums.iter().product::<usize>())
            } else {
                None
            }
        })
        .sum()
}

// returns all part numbers
// cleanup inspired by @zaquestion's day03 impl
fn adjacent_part_nums(graph: &Vec<Vec<Entry>>) -> impl Fn((usize, usize)) -> Vec<usize> + '_ {
    move |(y, x)| {
        vec![
            (y + 1, x),     // down
            (y - 1, x),     // up
            (y, x - 1),     // left
            (y, x + 1),     // right
            (y + 1, x + 1), // down right
            (y - 1, x + 1), // up right
            (y - 1, x - 1), // up left
            (y + 1, x - 1), // down left
        ]
        .iter()
        .filter(|(y, x)| {
            // only return numbers
            !(*y > graph.len() || *x > graph[*y].len())
                && matches!(graph[*y][*x], Entry::NumberPartial(_))
        })
        .map(|(y, x)| {
            // find the start of the number
            let num_start_x = (1..=*x)
                .rev()
                .skip_while(|x| matches!(graph[*y][*x - 1], Entry::NumberPartial(_)))
                .next()
                .unwrap_or_default();

            // find the end of the number
            let num_end_x = (*x..graph[*y].len())
                .skip_while(|x| matches!(graph[*y][*x], Entry::NumberPartial(_)))
                .next()
                .unwrap_or(graph[*y].len());

            graph[*y]
                .get(num_start_x..num_end_x)
                .expect("graph to have contain x's discovered earlier")
                .iter()
                .map(|e| match e {
                    Entry::NumberPartial(i) => *i,
                    Entry::Symbol(_) => panic!("not number"),
                    Entry::Empty => panic!("not number"),
                })
                .fold(0, |acc, num| acc * 10 + num)
        })
        .unique()
        .collect::<Vec<_>>()
    }
}

fn main() {
    let f = fs::File::open("day03/input.txt").expect("failed to open file");
    let reader = io::BufReader::new(f);

    let mut graph: Vec<Vec<char>> = vec![];

    // for part 2 I decided to mess w/ enums and have a "strongly typed"
    // data-structure to work with.
    let mut p2_graph: Vec<Vec<Entry>> = vec![];

    for l in reader.lines().into_iter() {
        let line = l.expect("read the lines");
        let chars = line.chars();

        let mut p1_vec: Vec<char> = vec![];
        let mut p2_vec: Vec<Entry> = vec![];
        for c in chars.into_iter() {
            p1_vec.push(c);

            match c {
                c if is_special_char(&c) => p2_vec.push(Entry::Symbol(c)),
                c if c == '.' => p2_vec.push(Entry::Empty),
                c if c.is_digit(10) => {
                    p2_vec.push(Entry::NumberPartial(c.to_digit(10).unwrap() as usize))
                }
                _ => panic!("wat"),
            }
        }

        graph.push(p1_vec);
        p2_graph.push(p2_vec);
    }

    // Iterate over the graph, finding numbers that have an adjacent
    // symbol next to them.

    // Correct: 539433
    println!("Problem 1: {}", part1(graph));

    // Correct: 75847567
    println!("Problem 2: {}", part2(p2_graph));
}
