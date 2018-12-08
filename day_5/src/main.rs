use std::collections::HashSet;

fn is_reaction(a: &char, b: &char) -> bool {
    a.eq_ignore_ascii_case(b) && a.is_ascii_uppercase() != b.is_ascii_uppercase()
}

fn react(polymer: Vec<char>) -> usize {
    let mut reduced : Vec<char> = Vec::new();
    for p in polymer {
        if reduced.last().map(|&c| is_reaction(&p, &c)).unwrap_or(false) {
            reduced.pop();
        } else {
            reduced.push(p.clone());
        }
    }

    reduced.len()
}

fn part_1_solve(input_str: &str) -> usize {
    react(input_str.chars().collect::<Vec<char>>())
}

fn part_2_solve(input_str: &str) -> usize {
    input_str.chars().fold(HashSet::new(), |mut set, c| {
        set.insert(c.to_ascii_lowercase());
        set
    }).iter().map(|u| {
        react(input_str.chars().filter(|c| {
            !c.eq_ignore_ascii_case(u)
        }).collect::<Vec<char>>())
    }).min().unwrap()
}

fn main() {
    println!("Part 1: {}", part_1_solve(include_str!("../input/input.txt")));
    println!("Part 2: {}", part_2_solve(include_str!("../input/input.txt")));
}

#[test]
fn test_reaction() {
    assert!(is_reaction(&'a',&'A'));
    assert!(!is_reaction(&'a',&'b'));
    assert!(!is_reaction(&'a',&'a'));
}

#[test]
fn part_1_test() {
    assert_eq!(part_1_solve(include_str!("../input/test_input_1.txt")), 10);
}

#[test]
fn part_2_test() {
    assert_eq!(part_2_solve(include_str!("../input/test_input_1.txt")), 4);
}