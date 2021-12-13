pub fn read_input() -> &'static str {
    include_str!("../input/day3.txt")
}

pub fn solve_1(input: &str) -> u32 {
    let count = input.lines().count();
    dbg!(count);
    let width = input.lines().next().unwrap().split("").count() - 2;
    dbg!(width);
    let ans = input
        .lines()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .fold(vec![0; width], |num, bits| {
            num.into_iter()
                .enumerate()
                .map(|(i, n)| n + ((bits & 1 << i) >> i))
                .collect()
        }) // returns a vector of count of bit == 1 in each position
        .into_iter()
        .enumerate()
        .map(|(i, b)| ((b >= count / 2) as u32) << i)
        .sum::<u32>();

    ans * (!ans & ((1 << width) - 1))
}

pub fn solve_2(input: &str) -> u32 {
    let count = input.lines().count();
    dbg!(count);
    let width = input.lines().next().unwrap().split("").count() - 2;
    dbg!(width);
    let ans = input
        .lines()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .fold(vec![0; width], |num, bits| {
            num.into_iter()
                .enumerate()
                .map(|(i, n)| n + ((bits & 1 << i) >> i))
                .collect()
        }) // returns a vector of count of bit == 1 in each position
        .into_iter()
        .enumerate()
        .map(|(i, b)| ((b >= count / 2) as u32) << i)
        .sum::<u32>();

    ans * (!ans & ((1 << width) - 1))
}

#[cfg(test)]
mod test {
    use crate::day3::solve_1;

    const TEST: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_first_problem() {
        let input = TEST;
        let expected = 198;
        let actual = solve_1(&input);
        assert_eq!(expected, actual);
    }

    //     #[test]
    //     fn test_second_problem() {
    //         let input = TEST;
    //         let expected = 230;
    //         let actual = solve_2(&input);
    //         assert_eq!(expected, actual);
    //     }
}
