use std::collections::HashSet;

fn part_1_solve(input_str: &str) -> i32 {
    input_str.lines().map(|x| x.parse::<i32>().unwrap()).sum()
}

fn part_2_solve(input_str: &str) -> i32 {
    let mut frequencies = HashSet::new();
    let mut frequency = 0;
 
    let _ = input_str.lines().cycle().map(|x| x.parse::<i32>().unwrap()).take_while(|x| {
        frequency += x;
        frequencies.insert(frequency)
    }).count();

    frequency
}

fn main() {
    println!("Part 1: {}", part_1_solve(include_str!("../input/input.txt")));
    println!("Part 2: {}", part_2_solve(include_str!("../input/input.txt")));
}

#[test]
fn part_2_test() {
    let mut test_str = "+3\n+3\n+4\n-2\n-4";
    assert_eq!(part_2_solve(test_str), 10);

    test_str = "-6\n+3\n+8\n+5\n-6";
    assert_eq!(part_2_solve(test_str), 5);

    test_str = "+7\n+7\n-2\n-7\n-4";
    assert_eq!(part_2_solve(test_str), 14);
}