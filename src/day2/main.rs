use std::fs;

const A: usize = 1;
const B: usize = 2;
const C: usize = 3;

enum Action {
    Lose = 1,
    Draw = 3,
    Win = 6,
    Undefined,
}

impl Action {
    fn from_str(action: &str) -> Action {
        match action {
            "X" => Action::Lose,
            "Y" => Action::Draw,
            "Z" => Action::Win,
            _ => Action::Undefined,
        }
    }
}

fn decide(oponent: usize, output: Action) -> usize {
    match output {
        Action::Lose => {
            if oponent == A {
                return C;
            } else if oponent == B {
                return A;
            } else {
                return B;
            }
        }
        Action::Draw => oponent + 3,
        Action::Win => {
            if oponent == A {
                return B + 6;
            } else if oponent == B {
                return C + 6;
            } else {
                return A + 6;
            }
        }
        _ => 0,
    }
}

fn transform(a: &str) -> usize {
    match a {
        "A" => A,
        "B" => B,
        "C" => C,
        _ => 0,
    }
}

fn main() {
    let contents =
        fs::read_to_string("res/day_2.txt").expect("Should have been able to read the file");

    let mut count: usize = 0;
    for round in contents.split("\n") {
        let result: usize = round
            .split(" ")
            .zip(round.split(" ").skip(1))
            .map(|(a, b)| return decide(transform(a), Action::from_str(b)))
            .sum();
        dbg!(result);
        count += result;
    }

    dbg!(count);
}
