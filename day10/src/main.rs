use std::collections::HashMap;

fn main () {
    // dbg!(part_a(include_str!("example.txt")));
    // dbg!(part_a(include_str!("input.txt")));
    // dbg!(part_b(include_str!("example.txt")));
    dbg!(part_b(include_str!("input.txt")));
}

pub fn part_a(input: &str) -> u32 {
    let points_map: HashMap<char, u32> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);

    let pairs: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);
    let opening_chars: Vec<char> = pairs.keys().map(|c| *c).collect();

    let mut score = 0u32;

    for line in input.trim().lines() {
        let mut stack: Vec<char> = vec![];

        for c in line.chars() {
            if opening_chars.contains(&c) {
                stack.push(c);
            }
            else if !stack.is_empty() && pairs[stack.last().unwrap()] == c{
                stack.pop();
            } else {
                println!("wrong char {}", c);
                score += points_map[&c];
                break;
            }
        }
        println!("{}", line);
    }

    score
}

pub fn part_b(input: &str) -> u64 {
    let points_map: HashMap<char, u64> = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);

    let pairs: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);
    let opening_chars: Vec<char> = pairs.keys().map(|c| *c).collect();

    let mut scores: Vec<u64> = vec![];
    
    for line in input.trim().lines() {
        let mut score = 0u64;

        let mut stack: Vec<char> = vec![];
        let mut incorrect = false;

        for c in line.chars() {
            if opening_chars.contains(&c) {
                stack.push(c);
            }
            else if !stack.is_empty() && pairs[stack.last().unwrap()] == c {
                stack.pop();
            } else {
                // println!("wrong char {}", c);
                // score += points_map[&c];
                incorrect = true;
                break;
            }
        }

        if !incorrect {
            // println!("{},   ", line);
            while let Some(c) = stack.pop() {
                // println!("{}", c);
                score *= 5;
                score += points_map[&c];
            }

            scores.push(score);
            // println!("{}", score);
            // println!();
        }

        // println!("{}", line);
    }

    scores.sort_unstable();

    scores[(scores.len() / 2)]
}
