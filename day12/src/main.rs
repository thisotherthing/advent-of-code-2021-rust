use std::collections::HashMap;

fn main () {
    // dbg!(part_a(include_str!("example_short.txt")));
    // dbg!(part_a(include_str!("example.txt")));
    // dbg!(part_a(include_str!("example_long.txt")));
    // dbg!(part_a(include_str!("input.txt")));
    // dbg!(part_b(include_str!("example_short.txt")));
    // dbg!(part_b(include_str!("example.txt")));
    // dbg!(part_b(include_str!("example_long.txt")));
    dbg!(part_b(include_str!("input.txt")));
}

struct Node {
    name: String,
    connections: Vec<usize>,
    small: bool,
}

fn add_node(name: &str, nodes: &mut Vec<Node>, node_ids: &mut HashMap<String, usize>) -> usize {
    if node_ids.contains_key(name) { return node_ids[name]; }


    let new_id = nodes.len();

    let small = name == name.to_lowercase();

    nodes.push(Node {
        name: name.to_string(),
        connections: vec![],
        small,
    });
    node_ids.insert(name.to_string(), new_id);

     new_id
}

fn traverse(
    node_id: usize,
    target_id: usize,
    nodes: &Vec<Node>,
    nodes_ids: &HashMap<String, usize>,
    num_paths: &mut u64,
    visited: &mut Vec<usize>
) {
    // println!("trav {}", visited.len());
    // println!("visited {:?}", visited);

    if node_id == target_id {
        println!("found path");
        print!("{}", nodes[visited[0]].name);
        
        for i in 1..visited.len() {
            print!(",{}", nodes[visited[i]].name);
        }
        println!();

        *num_paths += 1;
        return;
    }

    if nodes[node_id].small && visited.contains(&node_id) { return; }

    visited.push(node_id);

    for i in 0..nodes[node_id].connections.len() {
        let id = nodes[node_id].connections[i];
        // let node = &nodes[id];

        traverse(
            id,
            target_id,
            nodes,
            nodes_ids,
            num_paths,
            &mut visited.clone(),
        );
    }
}

pub fn part_a(input: &str) -> u64 {
    let mut num_paths: u64 = 0;

    let mut nodes: Vec<Node> = vec![];
    let mut nodes_ids: HashMap<String, usize> = HashMap::new();

    // load data
    {
        for line in input.trim().lines() {
            let parts: Vec<&str> = line
                .split('-')
                .map(|p| p.trim())
                .collect();

            let p0 = parts[0].to_string();
            let p1 = parts[1].to_string();

            let n0 = add_node(&p0, &mut nodes, &mut nodes_ids);
            let n1 = add_node(&p1, &mut nodes, &mut nodes_ids);

            nodes[n0].connections.push(n1);
            nodes[n1].connections.push(n0);
        }
    }

    let start: usize = nodes_ids["start"];
    let end: usize = nodes_ids["end"];

    println!("{:?}", nodes_ids);

    for i in 0..nodes.len() {
        println!("{}", nodes[i].name);
        for j in 0..nodes[i].connections.len() {
            print!(",{}", nodes[nodes[i].connections[j]].name);
        }
        println!()
    }

    traverse(
        start,
        end,
        &nodes,
        &nodes_ids,
        &mut num_paths,
        &mut vec![],
    );

    num_paths
}

fn traverse_b(
    node_id: usize,
    target_id: usize,
    nodes: &Vec<Node>,
    nodes_ids: &HashMap<String, usize>,
    num_paths: &mut u64,
    visited: &mut Vec<usize>,
    visited_count: &mut Vec<i32>
) {
    // println!("trav {}", visited.len());
    // println!("visited {:?}", visited);

    // println!("{} {:?}", visited.len(), visited_count);

    if nodes[node_id].small { visited_count[node_id] += 1; }
    if visited_count[node_id] > 2 { return; }
    if visited_count.iter().filter(|c| **c > 1).count() > 1 { return; }

    if node_id == target_id {
        println!("found path");
        print!("{}", nodes[visited[0]].name);
        
        for i in 1..visited.len() {
            print!(",{}", nodes[visited[i]].name);
        }
        println!();

        *num_paths += 1;
        return;
    }

    // println!("{} {:?}", visited.len(), visited_count);

    visited.push(node_id);

    for i in 0..nodes[node_id].connections.len() {
        let id = nodes[node_id].connections[i];
        // let node = &nodes[id];

        if id == nodes_ids["start"] { continue; }

        traverse_b(
            id,
            target_id,
            nodes,
            nodes_ids,
            num_paths,
            &mut visited.clone(),
            &mut visited_count.clone(),
        );
    }
}

pub fn part_b(input: &str) -> u64 {
    let mut num_paths: u64 = 0;

    let mut nodes: Vec<Node> = vec![];
    let mut nodes_ids: HashMap<String, usize> = HashMap::new();

    // load data
    {
        for line in input.trim().lines() {
            let parts: Vec<&str> = line
                .split('-')
                .map(|p| p.trim())
                .collect();

            let p0 = parts[0].to_string();
            let p1 = parts[1].to_string();

            let n0 = add_node(&p0, &mut nodes, &mut nodes_ids);
            let n1 = add_node(&p1, &mut nodes, &mut nodes_ids);

            nodes[n0].connections.push(n1);
            nodes[n1].connections.push(n0);
        }
    }

    let start: usize = nodes_ids["start"];
    let end: usize = nodes_ids["end"];

    println!("{:?}", nodes_ids);

    for i in 0..nodes.len() {
        println!("{}", nodes[i].name);
        for j in 0..nodes[i].connections.len() {
            print!(",{}", nodes[nodes[i].connections[j]].name);
        }
        println!()
    }

    let mut visited_count = vec![0i32; nodes.len()];

    // for i in 0..visited_count.len() {
    //     if !nodes[i].small { visited_count[i] = -9999; }
    // }

    // println!("{:?}", visited_count);

    traverse_b(
        start,
        end,
        &nodes,
        &nodes_ids,
        &mut num_paths,
        &mut vec![],
        & mut visited_count,
    );

    num_paths
}