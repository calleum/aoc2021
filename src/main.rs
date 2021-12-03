use std::fs::File;
use std::io::*;

fn main() {
    println!("{}", count("/Users/cal/dev/aoc2021/p1/input/depth.txt"));
}

fn count(file: &str) -> i32 {
    let depth_file = File::open(file).unwrap();
    let reader = BufReader::new(depth_file);
    let mut prev_line = 0;
    let mut current_line;
    let mut count = -1;

    for line in reader.lines() {
        current_line = line.unwrap().parse::<i32>().unwrap();
        if current_line > prev_line {
            count += 1;
        }
        prev_line = current_line
    }
    return count;
}
