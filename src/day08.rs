use std::cmp::{max, min};
use std::collections::HashMap;
use advent_2023::load_data;

struct Node<'a> {
    right: &'a str,
    left: &'a str,
    val : &'a str,
    start_node: bool,
    end_node: bool
}

impl<'a> Node<'a> {
    pub fn new(line: &'a str) -> Self {
        let val = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        Node {
            right,
            left,
            val,
            start_node: val.as_bytes()[2] == b'A',
            end_node: val.as_bytes()[2] == b'Z'
        }
    }
}

fn compute_cycles(p1: bool, instructions: &str, node_map: &HashMap<&str, Node>) -> usize{
    let mut cur_nodes: Vec<&Node> = if p1 {vec![node_map.get("AAA").unwrap()]} else {node_map.values().filter(|n| n.start_node).collect()};
    let mut cycles = vec![];
    let mut cycle = 0;
    while !cur_nodes.is_empty() {
        for ins in instructions.chars() {
            match ins {
                'L' => cur_nodes.iter_mut().for_each(|x| *x = node_map.get(x.left).unwrap()),
                'R' => cur_nodes.iter_mut().for_each(|x| *x = node_map.get(x.right).unwrap()),
                _ => panic!("Invalid ins")
            }
        }
        cycle += 1;
        let old_len = cur_nodes.len();
        cur_nodes = cur_nodes.iter().filter_map(|x| if !x.end_node {Some(*x)} else {None}).collect();
        if old_len != cur_nodes.len() {cycles.push(cycle)}
    }
    cycles.iter().fold(1_usize, |lcm, &c| least_common_multiple(lcm, c)) * instructions.len()
}

fn haunted_wasteland(data: &str) -> (usize, usize) {
    let instructions = data.split_once("\n").unwrap().0;

    let mut node_map: HashMap<&str, Node> = HashMap::new();
    for (idx, line) in data.lines().enumerate() {
        if idx < 2 { continue }
        let node = Node::new(line);
        node_map.insert(node.val, node);
    }

    let p2 = compute_cycles(false, instructions, &node_map);
    let p1 = compute_cycles(true, instructions, &node_map);
    (p1, p2)
}

fn greatest_common_factor(a: usize, b: usize) -> usize {
    let min_val = min(a, b);
    let max_val = max(a, b);
    if min_val == 0 {
        return max_val
    }
    greatest_common_factor(min_val, max_val % min_val)
}

fn least_common_multiple(a: usize, b: usize) -> usize {
    a * (b / greatest_common_factor(a, b))
}

pub fn day08() -> (usize, usize) {
    let data = load_data(8);
    haunted_wasteland(&data)
}

#[cfg(test)]
mod tests {
    use crate::day08::*;

    #[test]
    fn example_input() {
        let data = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let data2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(haunted_wasteland(data).0, 2);
        assert_eq!(haunted_wasteland(data2).0, 6);
    }

    #[test]
    fn p2_example() {
        let data = "LR

AAA = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(haunted_wasteland(data).1, 6)
    }

    #[test]
    fn test_start_end_node() {
        let node1 = Node::new("11A = (11B, XXX)");
        let node2 = Node::new("11B = (XXX, 11Z)");
        let node3 = Node::new("11Z = (11B, XXX)");
        assert!(node1.start_node && !node1.end_node);
        assert!(!node2.start_node && !node2.end_node);
        assert!(!node3.start_node && node3.end_node)
    }

    #[test]
    fn test_ans() {
        assert_eq!(day08(), (20659, 15_690_466_351_717))
    }
}