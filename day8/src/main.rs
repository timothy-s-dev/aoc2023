use std::collections::{HashMap};

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[derive(Copy, Clone, Debug)]
enum Direction { Left, Right }
impl Direction {
    fn parse(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction: {}", c),
        }
    }
}

#[derive(Debug)]
struct Node {
    label: String,
    id: usize,

    left_label: String,
    left_id: usize,

    right_label: String,
    right_id: usize,
}

struct HistoryEntry {
    node_id: usize,
    steps: usize,
    direction_offset: usize,
}

fn find_offsets(start_node_id: usize, nodes: &Vec<Node>, directions: Vec<Direction>) {
    let mut history: HashMap<(usize, usize), usize> = HashMap::new();
    let mut current_node_id = start_node_id;
    let mut steps = 0;
    loop {
        let direction_offset = steps % directions.len();
        let direction = directions[direction_offset];
        let next_node_id = match direction {
            Direction::Left => nodes[current_node_id].left_id,
            Direction::Right => nodes[current_node_id].right_id,
        };
        if nodes[next_node_id].label.ends_with("Z") {
            if history.contains_key(&(direction_offset, next_node_id)) {
                println!("Found a loop from {} - {} steps",
                         history.get(&(direction_offset, next_node_id)).unwrap(),
                         steps
                );
                break;
            }
        } else {
            history.insert((direction_offset, next_node_id), steps);
            println!("Updated History: {:?}", history);
        }
        steps += 1;
    }
}

fn main() {
    let is_part_one = common::is_part_one();
    let input_file_path = common::get_input_file_path();
    let lines = common::read_file_line_by_line(&input_file_path);

    let directions = lines[0].chars().map(|c| Direction::parse(c)).collect::<Vec<Direction>>();
    let mut nodes: Vec<Node> = Vec::new();
    let mut node_mapping: HashMap<String, usize> = HashMap::new();

    let mut next_id = 0;
    for line in lines[2..].iter() {
        let node = Node {
            label: line[0..3].to_string(),
            id: next_id,

            left_label: line[7..10].to_string(),
            left_id: 0,

            right_label: line[12..15].to_string(),
            right_id: 0,
        };
        node_mapping.insert(node.label.clone(), next_id);
        nodes.push(node);
        next_id += 1;
    }
    for node in nodes.iter_mut() {
        node.left_id = *node_mapping.get(&node.left_label).unwrap();
        node.right_id = *node_mapping.get(&node.right_label).unwrap();
    }

    if is_part_one {
        let mut steps: usize = 0;
        let mut current_node_id = *node_mapping.get("AAA").unwrap();
        while current_node_id != *node_mapping.get("ZZZ").unwrap() {
            let direction = directions[steps % directions.len()];
            current_node_id = match direction {
                Direction::Left => nodes[current_node_id].left_id,
                Direction::Right => nodes[current_node_id].right_id,
            };
            steps += 1;
        }

        println!("{:#?}", steps);
    } else {
        let mut starting_node_ids: Vec<usize> = Vec::new();
        for node in nodes.iter() {
            if node.label.ends_with("A") {
                starting_node_ids.push(node.id);
            }
        }
        let mut path_lengths: Vec<usize> = Vec::new();
        for starting_node_id in starting_node_ids.iter() {
            let mut steps: usize = 0;
            let mut current_node_id = starting_node_id;
            while !nodes[*current_node_id].label.ends_with("Z") {
                let direction = directions[steps % directions.len()];
                current_node_id = match direction {
                    Direction::Left => &nodes[*current_node_id].left_id,
                    Direction::Right => &nodes[*current_node_id].right_id,
                };
                steps += 1;
            }
            path_lengths.push(steps);
        }
        // get lcm of all path lengths
        let mut result = path_lengths[0];
        for i in 1..path_lengths.len() {
            result = lcm(result, path_lengths[i]);
        }
        println!("{:#?}", result);
    }
}
