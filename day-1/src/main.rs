use std::{fs, path::Path};

fn num_increases(depths: &Vec<u32>) -> u32 {
    let mut prev_depth = depths[0];
    let mut num_increase = 0;

    for depth in depths {
        if *depth > prev_depth {
            num_increase += 1;
        }
        prev_depth = *depth;
    }

    num_increase
}

fn main() {
    let filename = Path::new("./dat/input.dat");
    let contents = fs::read_to_string(filename).unwrap().trim().to_string();

    let split = contents.split("\n");
    let mut depths = Vec::new();

    for s in split {
        depths.push(s.parse::<u32>().unwrap());
    }

    // Part 1
    dbg!(num_increases(&depths));
}
