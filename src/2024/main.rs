use std::fs;

fn main() {
    let contents =
        fs::read_to_string("res/2024/day_1.txt").expect("Should have been able to read the file");
    let mut l: Vec<usize> = Vec::new();
    let mut r: Vec<usize> = Vec::new();
    for line in contents.lines() {
        l.push(usize::from_str_radix(line.split_ascii_whitespace().nth(0).unwrap(), 10).unwrap());
        r.push(usize::from_str_radix(line.split_ascii_whitespace().nth(1).unwrap(), 10).unwrap());
    }

    let mut similarity = 0;
    for a in l {
        let mut multiplier = 0;
        for b in &r {
            if a == *b {
                multiplier += 1;
            }
        }

        similarity += a * multiplier;
    }

    print!("{}", similarity);
}
