use std::collections::HashMap;
use std::cmp;

struct SumTuple {
    tuple_val: (u8, u8)
}

impl SumTuple {
    fn new(t: (u8,u8)) -> SumTuple {
        SumTuple{ tuple_val: t }
    }

    fn add(&self, to_add: (u8,u8)) -> SumTuple {
        SumTuple::new((self.tuple_val.0 + to_add.0, self.tuple_val.1 + to_add.1))
    }

    fn product(&self) -> i32 {
        self.tuple_val.0 as i32 * self.tuple_val.1 as i32
    }
}

fn part_1_solve(input_str: &str) -> i32 {
    input_str.lines().map(|line| {
        line.chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        }).values().fold((0u8, 0u8), |acc, x| {
            (cmp::max((*x == 2) as u8, acc.0), 
             cmp::max((*x == 3) as u8, acc.1))
        })
    }).fold(SumTuple::new((0u8,0u8)), |acc, t| acc.add(t)).product()
}

fn compare(a: &str, b: &str) -> bool {
    a.chars().zip(b.chars()).filter(|&(a,b)| a != b).count() == 1
}

fn part_2_solve(input_str: &str) -> Vec<char> {
    
    /*let lines_a = input_str.lines().collect::<Vec<&str>>();

    for next_str in lines_a {
        //let found = find_match(&next_str, compare_lines);
        let found = lines_a.to_vec().into_iter().find(|s| compare(&next_str, s));
        if found.is_some() {
            println!("found: {} <==> {}", next_str, found.unwrap());
        }
    }*/

    vec!('a')
}

fn main() {
    println!("Part 1: {}", part_1_solve(include_str!("../input/input.txt")));
    //println!("Part 2: {}", part_2_solve(include_str!("../input/input.txt")));
}

#[test]
fn part_1_test() {
    assert_eq!(part_1_solve(include_str!("../input/test_input_1.txt")), 12);
}

#[test]
fn part_2_test() {
    assert_eq!(part_2_solve(include_str!("../input/test_input_2.txt")), vec!('f','g','i','j'));
}