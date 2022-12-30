use std::fs;

#[derive(Debug)]
struct Range {
    pub lower: usize,
    pub upper: usize,
}

impl Range {
    pub fn new(lower: usize, upper: usize) -> Self {
        return Self { lower, upper };
    }

    pub fn contains(&self, other: &Range) -> bool {
        return (self.lower >= other.lower && self.lower <= other.upper)
            || (self.upper >= other.lower && self.upper <= other.upper);
    }
}

fn main() {
    let contents =
        fs::read_to_string("res/day_4.txt").expect("Should have been able to read the file");

    let mut pairs: Vec<(Range, Range)> = Vec::default();
    for line in contents.split("\n") {
        let mut s = line.split(",");
        let r1 = s.next().take().unwrap();
        let r2 = s.next().take().unwrap();

        let mut r1 = r1.split("-");
        let mut r2 = r2.split("-");

        let l1 = r1.next().take().unwrap();
        let u1 = r1.next().take().unwrap();
        let l2 = r2.next().take().unwrap();
        let u2 = r2.next().take().unwrap();

        let r1 = Range::new(l1.parse::<usize>().unwrap(), u1.parse::<usize>().unwrap());
        let r2 = Range::new(l2.parse::<usize>().unwrap(), u2.parse::<usize>().unwrap());
        pairs.push((r1, r2));
    }

    let result: usize = pairs
        .into_iter()
        .map(|(r1, r2)| {
            if r1.contains(&r2) || r2.contains(&r1) {
                return 1;
            } else {
                return 0;
            }
        })
        .sum();

    dbg!(result);
}
