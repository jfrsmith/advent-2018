fn parse_stream(stream: &Vec<u32>, node_start: usize) -> (u32, usize, u32) {
    let header = &stream[node_start..node_start+2];
    println!("Parse header => {:?}", header);

    let mut total = 0;
    let mut read_idx = node_start + 2;

    let child_vals = (0..header[0]).map(|_| {
        let (sum, read, val) = parse_stream(stream, read_idx);
        total += sum;
        read_idx = read;
        val
    }).collect::<Vec<u32>>();

    println!("Child node values: {:?}", child_vals);

    let end = read_idx + header[1] as usize;

    let (sum, value) = stream[read_idx..end].iter().fold((total,0), |acc, m| {
        let inner_sum = acc.0 + m;
        let inner_val = if child_vals.len() > 0 {
                match *m > 0 && *m <= child_vals.len() as u32 {
                    false => 0,
                    true => acc.1 + child_vals.get((*m-1) as usize).unwrap()
                }
            } else {
                acc.1 + m
            };

        (inner_sum, inner_val)
    });

    (sum, end, value)
}

fn part_1_solve(input_str: &str) -> u32 {
    let num_stream = input_str.split_whitespace().filter_map(|x| x.parse::<u32>().ok()).collect::<Vec<_>>();
    parse_stream(&num_stream, 0).0
}

fn part_2_solve(input_str: &str) -> u32 {
    let num_stream = input_str.split_whitespace().filter_map(|x| x.parse::<u32>().ok()).collect::<Vec<_>>();
    parse_stream(&num_stream, 0).2
}

fn main() {
    println!("Part 1: {}", part_1_solve(include_str!("../input/input.txt")));
    println!("Part 2: {}", part_2_solve(include_str!("../input/input.txt")));
}

#[test]
fn part_1_test() {
    assert_eq!(part_1_solve(include_str!("../input/test_input_1.txt")), 138);
}

#[test]
fn part_2_test() {
    assert_eq!(part_2_solve(include_str!("../input/test_input_1.txt")), 66);
}