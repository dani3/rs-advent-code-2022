use std::fs;
use std::vec::Vec;

fn main() {
    let contents =
        fs::read_to_string("res/input_day_1.txt").expect("Should have been able to read the file");

    let mut reindeers: Vec<usize> = Vec::new();
    let mut reindeer: usize = 0;
    for s in contents.split("\n") {
        if s.is_empty() {
            reindeers.push(reindeer);
            reindeer = 0
        } else {
            reindeer += s.parse::<usize>().unwrap();
        }
    }

    let mut max: [usize; 3] = [0; 3];
    for i in 0..3 {
        for r in reindeers.as_slice() {
            if r > &max[i] && !max.contains(r) {
                max[i] = *r;
            }
        }
    }

    dbg!(max[0] + max[1] + max[2]);
}
