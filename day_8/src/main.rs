fn parse_metadata(stream: &Vec<u32>, start: usize, end: usize, vals: &Vec<u32>) -> (u32, u32) {
    let metadata_sum = stream[start..end].iter().sum();
    let node_value = match vals.is_empty() {
        true => metadata_sum,
        false => stream[start..end].iter().fold(0, |acc, &m| {
            if m == 0 || m > (vals.len() as u32) {
                acc
            } else {
                acc + vals.get((m-1) as usize).unwrap()
            }
        })
    };

    (metadata_sum, node_value)
}

fn parse_stream(stream: &Vec<u32>, node_start: usize) -> (u32, usize, u32) {
    let header = &stream[node_start..node_start+2];
    let mut total = 0;
    let mut read_idx = node_start + 2;

    let child_vals = (0..header[0]).map(|_| {
        let (sum, read, val) = parse_stream(stream, read_idx);
        total += sum;
        read_idx = read;
        val
    }).collect::<Vec<u32>>();

    let end = read_idx + header[1] as usize;

    let (metadata_sum, node_value) = parse_metadata(stream, read_idx, end, &child_vals);

    (total + metadata_sum, end, node_value)
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