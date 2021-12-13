pub mod day3;

fn main() {
    println!("Problem 1: {}", day3::solve_1(&day3::read_input()));
    //println!("Problem 2: {}", day3::solve_2(&day3::read_input()));
    day3::solve_2(&day3::read_input());
}
