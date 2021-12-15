use std::{fs, path::Path};

fn compute_power_consumption(bins: &Vec<u64>) -> u64 {
    let mut gamma_rate = 0;
    let power = 12;
    let mut occ_ones = vec![0; power];

    for b in bins {
        for p in (0..power).rev() {
            let mask = 1 << p;
            occ_ones[p] += if b & mask > 0 { 1 } else { 0 };
        }
    }

    for p in (0..power).rev() {
        let mask = 1 << p;

        if occ_ones[p] > (bins.len() / 2) {
            gamma_rate += mask
        }
    }

    gamma_rate * ((!gamma_rate) & 0xfff)
}

fn main() {
    let filename = Path::new("./dat/input.dat");
    let contents = fs::read_to_string(filename).unwrap().trim().to_string();

    let split = contents.split("\n");
    let mut bins = Vec::new();

    for s in split {
        bins.push(u64::from_str_radix(s, 2).unwrap());
    }

    // Part 1
    dbg!(compute_power_consumption(&bins));
}
