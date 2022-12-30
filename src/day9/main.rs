use std::{collections::HashSet, fs};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Vec2 {
    pub x: isize,
    pub y: isize,
}

impl Vec2 {
    pub fn new(s: &str) -> Self {
        match s {
            "R" => Self { x: 0, y: 1 },
            "L" => Self { x: 0, y: -1 },
            "U" => Self { x: 1, y: 0 },
            "D" => Self { x: -1, y: 0 },
            _ => unreachable!(),
        }
    }

    pub fn r#move(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

fn main() {
    let contents =
        fs::read_to_string("res/day_9.txt").expect("Should have been able to read the file");

    let mut visited_nodes: HashSet<Vec2> = HashSet::default();

    let mut head: Vec2 = Vec2 { x: (0), y: (0) };
    let mut knots: [Vec2; 9] = [Vec2 { x: (0), y: (0) }; 9];
    for movement in contents.lines() {
        let (dir, steps) = movement.split_once(" ").unwrap();
        let steps: usize = steps.parse::<usize>().unwrap();
        for _ in 0..steps {
            let movement: Vec2 = Vec2::new(dir);
            head.r#move(movement);
            for i in 0..9 {
                let reference: Vec2;
                if i == 0 {
                    reference = head;
                } else {
                    reference = knots[i - 1];
                }
                let dist_x = reference.x - knots[i].x;
                let dist_y = reference.y - knots[i].y;

                if dist_x.abs() == 2 && dist_y == 0 {
                    // Horizontal
                    let xv = if dist_x > 0 { 1 } else { -1 };
                    knots[i].x += xv;
                } else if dist_y.abs() == 2 && dist_x == 0 {
                    // Vertocañ
                    let yv = if dist_y > 0 { 1 } else { -1 };
                    knots[i].y += yv;
                } else if (dist_x.abs() == 2 && dist_y.abs() > 0)
                    || (dist_y.abs() == 2 && dist_x.abs() > 0)
                {
                    let xv = if dist_x > 0 { 1 } else { -1 };
                    knots[i].x += xv;
                    let yv = if dist_y > 0 { 1 } else { -1 };
                    knots[i].y += yv;
                }
            }

            visited_nodes.insert(knots[8]);
        }
    }

    println!("{0}", visited_nodes.len());
}
