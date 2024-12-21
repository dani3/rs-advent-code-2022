use std::fs;

fn main() {
    let contents =
        fs::read_to_string("res/day_6.txt").expect("Should have been able to read the file");

    let mut found: bool;
    let mut index = 0;
    for i in 0..contents.len() {
        found = true;
        let sub = &contents[i..i + 14];
        for c in sub.chars() {
            if sub.matches(c).count() > 1 {
                found = false;
                break;
            }
        }

        if found {
            index = i + 14;
            break;
        }
    }

    dbg!(index);
}
