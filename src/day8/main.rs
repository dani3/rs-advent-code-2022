use std::fs;

fn main() {
    let contents =
        fs::read_to_string("res/day_8.txt").expect("Should have been able to read the file");

    let mut trees: Vec<Vec<usize>> = Vec::default();
    for (i, line) in contents.lines().enumerate() {
        trees.push(Vec::default());
        for tree in line.chars() {
            trees[i].push(tree.to_digit(10).unwrap() as usize);
        }
    }

    let rows = trees.len();
    let columns = trees[0].len();
    let mut max: usize = 0;
    for i in 1..(rows - 1) {
        for j in 1..(columns - 1) {
            // Check above
            let mut above: usize = 0;
            for z in (0..i).rev() {
                above += 1;
                if trees[i][j] <= trees[z][j] {
                    break;
                }
            }
            // Check below
            let mut below: usize = 0;
            for z in (i + 1)..rows {
                below += 1;
                if trees[i][j] <= trees[z][j] {
                    break;
                }
            }
            // Check left
            let mut left: usize = 0;
            for z in (0..j).rev() {
                left += 1;
                if trees[i][j] <= trees[i][z] {
                    break;
                }
            }
            // Check right
            let mut right: usize = 0;
            for z in (j + 1)..columns {
                right += 1;
                if trees[i][j] <= trees[i][z] {
                    break;
                }
            }

            let scenic_score = above * below * right * left;
            if max < scenic_score {
                max = scenic_score;
            }
        }
    }

    dbg!(max);
}
