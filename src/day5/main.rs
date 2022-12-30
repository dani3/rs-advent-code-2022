use std::{collections::LinkedList, fs};

const EMPTY_CRATE: usize = 4;

fn main() {
    let contents =
        fs::read_to_string("res/day_5.txt").expect("Should have been able to read the file");
    let mut lines = contents.split("\n");

    let mut stacks: [LinkedList<char>; 9] = Default::default();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let mut spaces: usize = 0;
        let mut next_index: usize = 0;
        for c in line.chars() {
            match c {
                ' ' => {
                    spaces += 1;
                }
                'A'..='Z' => {
                    let index = spaces / EMPTY_CRATE;
                    spaces = 0;
                    stacks[index + next_index].push_front(c);
                    next_index += index + 1;
                }
                _ => {}
            }
        }
    }

    for line in lines {
        let mut moves = line
            .split_whitespace()
            .filter(|s| return s.contains(|c: char| return c.is_digit(10)))
            .map(|s| s.parse::<usize>());

        match (moves.next(), moves.next(), moves.next()) {
            (Some(Ok(quantity)), Some(Ok(from)), Some(Ok(to))) => {
                let mut aux = LinkedList::default();
                for _ in 0..quantity {
                    let e = stacks[from - 1].pop_back().unwrap();
                    aux.push_front(e);
                }

                for _ in 0..quantity {
                    let e = aux.pop_front().unwrap();
                    stacks[to - 1].push_back(e);
                }
            }
            (_, _, _) => {}
        }
    }

    for stack in stacks {
        let top = stack.back().unwrap();
        print!("{top}")
    }
}
