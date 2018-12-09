use std::cmp;
use std::collections::BTreeMap;

type PreReqVec = Vec<char>;
type NodeMap = BTreeMap<char, PreReqVec>;

#[derive(Debug, Clone)]
struct Job {
    node: char,
    end_time: u32
}

#[derive(Debug, Clone)]
struct Worker {
    queue: Vec<Job>
}

impl Worker {
    fn get_node_end_time(&self, node: char) -> Option<u32> {
        self.queue.iter().find(|j| j.node == node).and_then(|j| Some(j.end_time))
    }

    fn get_earliest_start_time(&self) -> u32 {
        self.queue.iter().map(|j| j.end_time).last().unwrap_or(0)
    }

    fn schedule(&mut self, n: char, not_before: u32, duration: u32) {
        //println!("\t\tQueue => {:?}", self.queue);
        let start_at = cmp::max(not_before, self.get_earliest_start_time());
        let d = get_node_time(n, duration);
        let e = start_at + get_node_time(n, duration);
        //println!("\t\tScheduling node {} to start at {} with duration {}", n, start_at, d);
        self.queue.push(Job { 
            node: n, 
            end_time: e
        });
        //println!("\t\tQueue => {:?}", self.queue);
    }

    fn bid_node(&self, start_time: u32) -> u32 {
        let e = self.get_earliest_start_time();
        cmp::max(e, start_time) - start_time
    }

    fn node_at_time(&self, time: u32, duration: u32) -> Option<char> {
        self.queue.iter().find(|j| {
            time >= (j.end_time - get_node_time(j.node, duration)) && time < j.end_time
        }).and_then(|j| Some(j.node))
    }
}

struct ElfPool {
    workers : Vec<Worker>,
    duration : u32, 
    nodes : NodeMap
}

impl ElfPool {
    fn new(num_workers: u32, dur: u32, map: &NodeMap) -> ElfPool {
        ElfPool { 
            workers : vec!(Worker { queue: vec!() } ; num_workers as usize), 
            duration : dur,
            nodes: map.to_owned()
        }
    }

    fn get_node_end_time(&self, node: char) -> Option<u32> {
        self.workers.iter().find_map(|w| w.get_node_end_time(node))
    }

    fn is_node_queued(&self, node: char) -> bool {
        self.workers.iter().any(|w| w.get_node_end_time(node).is_some())
    }

    fn can_schedule(&mut self, node: char) -> bool {
         !self.is_node_queued(node) &&
         self.nodes.get(&node).unwrap_or(&PreReqVec::new()).iter().all(|p| self.is_node_queued(*p))
    }

    fn schedule(&mut self, node: char) {
        //println!("Attempting to schedule node {}", node);
        let start_time = self.nodes.get(&node).unwrap_or(&PreReqVec::new()).iter().filter_map(|n| {
            self.get_node_end_time(*n)
        }).max().unwrap_or(0);
       // println!("\tEarliest possible start time is {}", start_time);
        (*self.workers.iter_mut().min_by_key(|w| w.bid_node(start_time)).unwrap()).schedule(node, start_time, self.duration); 
    }

    fn resolve(&self) -> u32 {
        self.workers.iter().map(|w| w.get_earliest_start_time()).max().unwrap()
    }

    fn print(&self) {
        for x in 0..=self.resolve() {
            print!("{:04}", x);
            for job in self.workers.iter().map(|w| w.node_at_time(x, self.duration)) {
                let c = match job {
                    Some(n) => n,
                    None => '.'
                };
                print!("\t{}", c);
            }
            println!("");
        }
    }
}

fn get_node_time(node: char, base_time: u32) -> u32 {
    assert!(node.is_alphabetic() && node.is_uppercase());
    (node as u8 - 64) as u32 + base_time
}

fn build_node_map(input_str: &str) -> NodeMap {
    input_str.lines().fold(NodeMap::new(), |mut map, l| {
        let (pre_req, node) = (l.chars().nth(5).unwrap(), l.chars().nth(36).unwrap());
        
        if !map.contains_key(&pre_req) {
            map.insert(pre_req, vec!());
        }

        (*map.entry(node).or_insert(PreReqVec::new())).push(pre_req);
        map
    })
}

fn is_node_available(node: &char, nodes: &NodeMap, visited: &Vec<char>) -> bool {
    !visited.contains(node) && nodes.get(node).unwrap().iter().all(|n| visited.contains(n))
}

fn traverse_nodes(nodes: &NodeMap) -> String {
    let mut visited : Vec<char> = vec!();
    let mut keys = nodes.keys().cloned().collect::<Vec<char>>();

    while !keys.is_empty() {
        let found_idx = keys.iter().enumerate().find(|x| is_node_available(x.1, nodes, &visited)).unwrap().0;
        visited.push(keys.drain(found_idx..found_idx+1).nth(0).unwrap());
    }

    visited.iter().collect::<String>()
}

fn part_1_solve(input_str: &str) -> String {
    traverse_nodes(&build_node_map(input_str))
}

fn traverse_nodes_parallel(nodes: &NodeMap, num_workers: u32, duration: u32) -> u32 {
    let mut pool = ElfPool::new(num_workers, duration, nodes);

    while nodes.keys().any(|n| !pool.is_node_queued(*n)) {
        let schedule_keys = nodes.keys().cloned().filter(|k| pool.can_schedule(*k)).collect::<Vec<char>>();
        //println!("== Schedule Pass for {:?} ==", schedule_keys);
        for n in schedule_keys {
            pool.schedule(n);
        }
        //println!("=================================");
    }

    pool.print();

    pool.resolve()
}

fn part_2_solve(input_str: &str, num_workers: u32, duration: u32) -> u32 {
    traverse_nodes_parallel(&build_node_map(input_str), num_workers, duration)
}

fn main() {
    println!("Part 1: {}", part_1_solve(include_str!("../input/input.txt")));
    println!("Part 2: {}", part_2_solve(include_str!("../input/input.txt"), 5, 60));
}

#[test]
fn part_1_test() {
    assert_eq!(part_1_solve(include_str!("../input/test_input_1.txt")), "CABDFE");
}

#[test]
fn part_2_test() {
    assert_eq!(part_2_solve(include_str!("../input/test_input_1.txt"), 2, 0), 15);
    assert_eq!(get_node_time('A', 0), 1);
    assert_eq!(get_node_time('Z', 0), 26);
}