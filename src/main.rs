use std::fs::File;
use std::io::*;

fn main() {
    println!("{}", count("/Users/cal/dev/aoc2021/p1/input/depth.txt"));
}

fn forward(amount: i32) -> i32 {}

fn count(file: &str) -> i32 {
    let depth_file = File::open(file).unwrap();
    let reader = BufReader::new(depth_file);
    let mut current_line;
    let mut prev_sum = 0;
    let mut sum;
    let mut count = -3;
    let mut prev_line = -1;
    let mut prev_prev_line = -1;

    for line in reader.lines() {
        current_line = line.unwrap().parse::<i32>().unwrap();
        sum = current_line + prev_line + prev_prev_line;
        if sum > prev_sum {
            count += 1;
        }
        prev_sum = sum;
        prev_prev_line = prev_line;
        prev_line = current_line;
    }
    return count;
}
