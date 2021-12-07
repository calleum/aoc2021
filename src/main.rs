use std::fs::File;
use std::io::*;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    aim: i32,
}

fn main() {
    println!("{}", count("/Users/cal/dev/aoc2021/input/instructions"));
}

fn forward(p: &mut Point, amount: i32) {
    println!("{:?}", p);
    p.x += amount;
    p.y += p.aim * amount;
}

fn up(p: &mut Point, amount: i32) {
    p.aim -= amount
}

fn down(p: &mut Point, amount: i32) {
    p.aim += amount
}

fn lines_iter(b: BufReader<File>, mut p: &Point, f: &mut dyn FnMut(String, &Point)) {
    let mut i = 0;
    for line in b.lines() {
        i += 1;
        let l = line.unwrap();
        f(l, p);
    }
}

fn match_instr(l: String, mut p: &Point) {
    let instr: Vec<&str> = l.split(' ').collect();
    match instr[..] {
        ["forward", x] => forward(&mut p, x.parse::<i32>().unwrap()),
        ["down", x] => down(&mut p, x.parse::<i32>().unwrap()),
        ["up", x] => up(&mut p, x.parse::<i32>().unwrap()),
        _ => eprintln!("whoops"),
    };
}

fn count(file: &str) -> i32 {
    let depth_file = File::open(file).unwrap();
    let b = BufReader::new(depth_file);
    let mut p = Point { x: 0, y: 0, aim: 0 };
    let mut i = 0;

    lines_iter(b, &p, match_instr);

    return p.x * p.y;
}
