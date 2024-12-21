use std::fs;

fn convert(c: char) -> usize {
    if c.is_uppercase() {
        return c as usize - 38;
    } else {
        return c as usize - 96;
    }
}

fn main() {
    let contents =
        fs::read_to_string("res/day_3.txt").expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut result: usize = 0;
    for i in (0..lines.len()).step_by(3) {
        let a: &str = lines[i];
        let b: &str = lines[i + 1];
        let c: &str = lines[i + 2];

        dbg!(a);
        dbg!(b);
        dbg!(c);

        for char in a.chars() {
            if b.contains(char) && c.contains(char) {
                result += convert(char);
                break;
            }
        }
    }

    dbg!(result);
}
