use std::{collections::HashMap};

fn main () {
    // dbg!(part_a(include_str!("example.txt")));
    // dbg!(part_a(include_str!("input.txt")));
    // dbg!(part_b(include_str!("example.txt")));
    dbg!(part_b(include_str!("input.txt")));
}

pub fn part_a(input: &str) -> i32 {

    let crabs: Vec<i32> =
        input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
        ;

    let min= *crabs.iter().min().unwrap() as i32;
    let max= *crabs.iter().max().unwrap() as i32;

    let mut min_cost = i32::MAX;

    for i in min..max {
        let mut cost = 0;

        for crab in &crabs {
            cost += (crab - i).abs();
        }

        if min_cost > cost { min_cost = cost; }
    }

    // dbg!(count);
    // dbg!(boards);

    min_cost
}

fn get_cost(num_steps: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if memo.contains_key(&num_steps) { return memo[&num_steps]; }

    let mut cost = 0;

    for i in 1..num_steps {
        cost += i;
        memo.entry(i).or_insert(cost);
    }

    memo.entry(num_steps).or_insert(cost);

    cost
}

pub fn part_b(input: &str) -> i32 {

    let crabs: Vec<i32> =
        input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
        ;

    let min= *crabs.iter().min().unwrap() as i32;
    let max= *crabs.iter().max().unwrap() as i32;

    let mut min_cost = i32::MAX;

    let mut memo: HashMap<i32, i32> = HashMap::new();

    for i in min..max {
        let mut cost = 0;

        for crab in &crabs {
            cost += get_cost((crab - i).abs(), &mut memo);
        }

        if min_cost > cost { min_cost = cost; }
    }

    // dbg!(count);
    // dbg!(boards);

    min_cost
}
