use std::collections::HashMap;

type PreReqVec = Vec<char>;
type NodeMap = HashMap<char, PreReqVec>;

fn parse_instruction(instr: &str) -> (char, char) {
    let lower: String = instr.to_lowercase();
    let split = lower.split("step").collect::<Vec<&str>>();
    (split[1].trim().chars().nth(0).unwrap(), split[2].trim().chars().nth(0).unwrap())
}

fn build_node_map(input_str: &str) -> NodeMap {
    input_str.lines().fold(NodeMap::new(), |mut map, l| {
        let (pre_req, node) = parse_instruction(l);
        
        //add the pre-requisite first as it may not end up being added if it's the root
        if !map.contains_key(&pre_req) {
            map.insert(pre_req, vec!());
        }

        //update this node with a pre-req
        let pre_req_vec = map.entry(node).or_insert(PreReqVec::new());
        *pre_req_vec.push(pre_req);
        map
    })
}

fn part_1_solve(input_str: &str) -> String {
    println!("{:?}", build_node_map(input_str));
    "".to_string()
}

fn main() {
    println!("Part 1: {}", part_1_solve(include_str!("../input/input.txt")));
    //println!("Part 2: {}", part_2_solve(include_str!("../input/input.txt"), 10000));
}

#[test]
fn part_1_test() {
    assert_eq!(part_1_solve(include_str!("../input/test_input_1.txt")), "CABDFE");
}

/*#[test]
fn part_2_test() {
    assert_eq!(part_2_solve(include_str!("../input/test_input_1.txt"), 32), 16);
}*/