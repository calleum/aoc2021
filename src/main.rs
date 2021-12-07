
fn read_input1() -> String {
    include_str!("../input/input1.txt").to_string()
}

fn read_input2() -> String {
    include_str!("../input/input2.txt").to_string()
}

fn solve_1(input: &str) -> isize {
    let gamma = input
        .lines()
        .filter_map(|m| m.ok())
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .fold(vec![0; 12], |num, bits| 
            num
              );
   todo!() 
}

fn solve_2(input: &str) -> isize {
   todo!() 
}

fn main() {
    println!("Solution 1: {:?}", solve_1(&read_input1()));
    println!("Solution 2: {:?}", solve_2(&read_input2()));
}

#[cfg(test)]
mod test {
    use crate::{solve_1, solve_2};

    const TEST: &str = "10100
    10110
    00111
    10101
    01111
    11110
    11100
    10111
    11001
    00010
    10000
    01010";

    #[test]
    fn test_first_problem() {
        let input = TEST;
        let expected = 198;
        let actual = solve_1(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_second_problem() {
        let input = TEST;
        let expected = 230;
        let actual = solve_2(&input);
        assert_eq!(expected, actual);
    }
}
