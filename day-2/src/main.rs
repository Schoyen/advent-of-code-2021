use std::{fs, path::Path};

fn mul_sum_directions(dir: &Vec<&str>, amount: &Vec<u32>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    assert_eq!(dir.len(), amount.len());

    for i in 0..dir.len() {
        if dir[i] == "forward" {
            horizontal += amount[i] as i32;
        } else if dir[i] == "up" {
            depth -= amount[i] as i32;
        } else if dir[i] == "down" {
            depth += amount[i] as i32;
        } else {
            dbg!("OH BOI");
        }
    }

    horizontal * depth
}

fn main() {
    let filename = Path::new("./dat/input.dat");
    let contents = fs::read_to_string(filename).unwrap().trim().to_string();

    let split = contents.split("\n");
    let mut dir = Vec::new();
    let mut amount = Vec::new();

    for s in split {
        let mut lines = s.split_whitespace();
        dir.push(lines.next().unwrap());
        amount.push(lines.next().unwrap().parse::<u32>().unwrap());
    }

    // Part 1
    dbg!(mul_sum_directions(&dir, &amount));
}
