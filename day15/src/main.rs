use std::{collections::HashMap, ops::Rem};

fn main () {
    // dbg!(part_a(include_str!("example.txt")));
    // dbg!(part_a(include_str!("input.txt")));
    // dbg!(part_a(include_str!("example_partb.txt")));
    // dbg!(part_b(include_str!("example.txt")));
    // dbg!(part_b(include_str!("input.txt"), 1));
    // dbg!(part_b(include_str!("example.txt"), 5));
    dbg!(part_b(include_str!("input.txt"), 5));
}

// fn walk(x: usize, y: usize, w: usize, h: usize, current_path: &mut Vec<u64>, level: &Vec<Vec<u64>>, paths: &mut Vec<u64>) {
//     if x == w && y == h {
//         let mut cost = current_path.iter().sum();
//         cost += level[y][x];
//         paths.push(cost);
//         return;
//     }

//     current_path.push(level[y][x]);

//     if x < w { walk(x + 1, y, w, h, current_path, level, paths); }
//     if y < h { walk(x, y + 1, w, h, current_path, level, paths); }

//     current_path.pop();
// }

// fn walk_cheapest(x: usize, y: usize, w: usize, h: usize, current_path: &mut Vec<u64>, level: &Vec<Vec<u64>>, paths: &mut Vec<u64>) {
//     if x == w && y == h {
//         let mut cost = current_path.iter().sum();
//         cost += level[y][x];
//         paths.push(cost);
//         return;
//     }

//     current_path.push(level[y][x]);

//     if x < w && level[y][x+1] < level[y + 1][x] {
//         walk_cheapest(x + 1, y, w, h, current_path, level, paths);
//     } else {
//         walk_cheapest(x, y + 1, w, h, current_path, level, paths);
//     }

//     current_path.pop();
// }

// fn walk_cheapest(x: usize, y: usize, w: usize, h: usize, current_path: &mut Vec<u64>, level: &Vec<Vec<u64>>, paths: &mut Vec<u64>) {
    
// }
fn check_if_move(
    point: (usize, usize),
    current_cost: u64,
    cost_map: &mut HashMap<(usize,usize), u64>,
    level: &Vec<Vec<u64>>,
    paths: &mut Vec<((usize, usize), u64)>,
    w: usize,
    h: usize
) {
    let mut insert = false;

    if point.0 <= w &&
        point.1 <= h
    {
        if cost_map.contains_key(&point) {
            insert = current_cost + level[point.1][point.0] < cost_map[&point];
        } else {
            insert = true;
        }
    }

    if insert {
        paths.push(
            (
                point,
                current_cost + level[point.1][point.0]
            )
        );
        *cost_map.entry(point).or_insert(0) = current_cost + level[point.1][point.0];
    }
}

pub fn part_a(input: &str) -> u64 {

     let mut level: Vec<Vec<u64>> = vec![];


    for line in input.trim().lines() {
        level.push(
            line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect()
        );
    }

    let mut cost_map: HashMap<(usize,usize), u64> = HashMap::new();

    let mut paths: Vec<((usize, usize), u64)> = vec![((0, 0), 0)];
    // let mut current_path: Vec<u64> = vec![];
    // walk(0, 0, level[0].len() - 1, level.len() - 1, &mut current_path, &level, &mut paths);
    // walk_cheapest(0, 0, level[0].len() - 1, level.len() - 1, &mut current_path, &level, &mut paths);

    let target_pos = ((level[0].len() - 1), (level.len() - 1));

    let mut tmp_point = (0usize, 0usize);
    
    let mut step = 0;

    loop {
        paths.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        // for i in (0..paths.len().min(10)).rev() {
        for i in (0..paths.len()).rev() {
            let point = paths[i];
            let tmp_cost = point.1;

            // left
            if point.0.0 > 0 {
                tmp_point.0 = point.0.0 - 1;
                tmp_point.1 = point.0.1;
                check_if_move(tmp_point, tmp_cost, &mut cost_map, &level, &mut paths, target_pos.0, target_pos.1)
            }
            // right
            {
                tmp_point.0 = point.0.0 + 1;
                tmp_point.1 = point.0.1;
                check_if_move(tmp_point, tmp_cost, &mut cost_map, &level, &mut paths,target_pos.0, target_pos.1);
            }

            // up
            if point.0.1 > 0 {
                tmp_point.0 = point.0.0;
                tmp_point.1 = point.0.1 - 1;
                check_if_move(tmp_point, tmp_cost, &mut cost_map, &level, &mut paths,target_pos.0, target_pos.1);
            }
            // down
            {
                tmp_point.0 = point.0.0;
                tmp_point.1 = point.0.1 + 1;
                check_if_move(tmp_point, tmp_cost, &mut cost_map, &level, &mut paths,target_pos.0, target_pos.1);
            }
            

            paths.remove(i);
        }

        if cost_map.contains_key(&target_pos) {
            break;
        }

        step += 1;

        println!("{} {}", step, paths.len());
    } 

    cost_map[&target_pos]
}

fn get_level_at(
    point: &(usize, usize),
    level: &Vec<Vec<u64>>,
) -> u64 {
    let w = level[0].len();
    let h = level.len();
    let mut val = level[point.1.rem(h)][point.0.rem(w)];

    val += (point.0 / w) as u64;
    val += (point.1 / h) as u64;

    (val - 1).rem(9) + 1
}

fn check_if_move_b(
    point: (usize, usize),
    current_cost: u64,
    cost_map: &mut HashMap<(usize,usize), u64>,
    level: &Vec<Vec<u64>>,
    paths: &mut Vec<((usize, usize), u64)>,
    w: usize,
    h: usize
) {
    let mut insert = false;

    if point.0 <= w &&
        point.1 <= h
    {
        if cost_map.contains_key(&point) {
            insert = current_cost + get_level_at(&point, level) < cost_map[&point];
        } else {
            insert = true;
        }
    }

    if insert {
        paths.push(
            (
                point,
                current_cost + get_level_at(&point, level)
            )
        );
        *cost_map.entry(point).or_insert(0) = current_cost + get_level_at(&point, level);
    }
}

pub fn part_b(input: &str, scaler: usize) -> u64 {

     let mut level: Vec<Vec<u64>> = vec![];


    for line in input.trim().lines() {
        level.push(
            line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect()
        );
    }

    let mut cost_map: HashMap<(usize,usize), u64> = HashMap::new();

    let mut paths: Vec<((usize, usize), u64)> = vec![((0, 0), 0)];
    // let mut current_path: Vec<u64> = vec![];
    // walk(0, 0, level[0].len() - 1, level.len() - 1, &mut current_path, &level, &mut paths);
    // walk_cheapest(0, 0, level[0].len() - 1, level.len() - 1, &mut current_path, &level, &mut paths);

    // let target_pos = ((level[0].len() * 5 - 1), (level.len() * 5 - 1));
    let target_pos = ((level[0].len() * scaler - 1), (level.len() * scaler - 1));

    // let full_size = level[0].len() * scaler * level.len() * scaler;

    let mut tmp_point = (0usize, 0usize);
    
    let mut step = 0;

    loop {
        paths.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        // for i in (0..paths.len().min(10)).rev() {
        for i in (0..paths.len()).rev() {
            let point = paths[i];
            let tmp_cost = point.1;

            // left
            if point.0.0 > 0 {
                tmp_point.0 = point.0.0 - 1;
                tmp_point.1 = point.0.1;
                check_if_move_b(tmp_point, tmp_cost, &mut cost_map, &level, &mut paths, target_pos.0, target_pos.1)
            }
            // right
            {
                tmp_point.0 = point.0.0 + 1;
                tmp_point.1 = point.0.1;
                check_if_move_b(tmp_point, tmp_cost, &mut cost_map, &level, &mut paths,target_pos.0, target_pos.1);
            }

            // up
            if point.0.1 > 0 {
                tmp_point.0 = point.0.0;
                tmp_point.1 = point.0.1 - 1;
                check_if_move_b(tmp_point, tmp_cost, &mut cost_map, &level, &mut paths,target_pos.0, target_pos.1);
            }
            // down
            {
                tmp_point.0 = point.0.0;
                tmp_point.1 = point.0.1 + 1;
                check_if_move_b(tmp_point, tmp_cost, &mut cost_map, &level, &mut paths,target_pos.0, target_pos.1);
            }
            

            paths.remove(i);
        }

        // if cost_map.contains_key(&target_pos) {
        // if cost_map.len() == full_size {
        if paths.is_empty() {
            break;
        }

        step += 1;

        println!("{} {}", step, paths.len());
    } 

    cost_map[&target_pos]
}
