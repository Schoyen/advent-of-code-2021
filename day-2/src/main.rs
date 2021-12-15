use std::{fs, path::Path};

fn mul_sum_directions(dir: &Vec<&str>, amount: &Vec<i64>) -> i64 {
    let mut horizontal = 0;
    let mut depth = 0;

    assert_eq!(dir.len(), amount.len());

    for i in 0..dir.len() {
        if dir[i] == "forward" {
            horizontal += amount[i];
        } else if dir[i] == "up" {
            depth -= amount[i];
        } else if dir[i] == "down" {
            depth += amount[i];
        } else {
            dbg!("OH BOI");
        }
    }

    horizontal * depth
}

fn mul_sum_aim(dir: &Vec<&str>, amount: &Vec<i64>) -> i64 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    assert_eq!(dir.len(), amount.len());

    for i in 0..dir.len() {
        if dir[i] == "forward" {
            horizontal += amount[i];
            depth += aim * amount[i];
        } else if dir[i] == "up" {
            aim -= amount[i];
        } else if dir[i] == "down" {
            aim += amount[i];
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
        amount.push(lines.next().unwrap().parse::<i64>().unwrap());
    }

    // Part 1
    dbg!(mul_sum_directions(&dir, &amount));

    // Part 2
    dbg!(mul_sum_aim(&dir, &amount));
}
