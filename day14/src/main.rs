use std::collections::HashMap;

fn main () {
    // dbg!(part_a(include_str!("example.txt"), 10));
    // dbg!(part_a(include_str!("input.txt"), 10));
    // dbg!(part_b(include_str!("example.txt"), 40));
    dbg!(part_b(include_str!("input.txt"), 40));
}

fn increase_char_counts_for_pairs(c0: char, c1:char, steps: usize, pairs: &HashMap<String, char>, memo: &mut HashMap<(char, char, usize), Vec<u64>>, counts: &mut Vec<u64>) {
    if steps == 0 { return; }

    let mut key = String::from("");
    key.push(c0);
    key.push(c1);

    let memo_key = &(c0, c1, steps);

    if !memo.contains_key(memo_key) {
        let mut local_count = vec![0; 26];
    
        if pairs.contains_key(&key) {
            let insert = pairs[&key];
            local_count[insert as usize - 65] += 1;
    
            increase_char_counts_for_pairs(c0, insert, steps - 1, pairs, memo, &mut local_count);
            increase_char_counts_for_pairs(insert, c1, steps - 1, pairs, memo, &mut local_count);
        }
    
        memo.insert(*memo_key, local_count);
    }




    for i in 0..counts.len() {
        counts[i] += memo[memo_key][i];
    }
}
// fn increase_char_counts_for_pairs(c0: char, c1:char, steps: usize, pairs: &HashMap<String, char>, memo: &mut HashMap<(char, char, usize), Vec<u64>>, result: &mut Vec<u64>) {
//     if memo.contains_key(&(c0, c1, steps)) {
//         for i in 0..memo[&(c0, c1, steps)].len() {
//             result[i] += memo[&(c0, c1, steps)][i];
//         }
//         return;
//     }

//     let tmp = vec![0, 90];


//     let mut key = String::from("");

//     for step in 0..steps {
//         for i in (1..template.len()).rev() {
//             if steps < 5 {
//                 key.clear();
//                 key.push(template[i-1]);
//                 key.push(template[i]);
        
//                 if pairs.contains_key(&key) {
//                     template.insert(i, pairs[&key]);
//                 }
//             } else {
//                 increase_char_counts_for_pairs(
//                     template[i-1],
//                     template[i],
//                     1,
//                     pairs,
//                     memo,
//                     result,
//                 );
//             }

//         }
//     }

//     for c in template {
//         result[c as usize] += 1;
//     }

//     memo.insert((c0, c1, steps), tmp);
// }

pub fn part_a(input: &str, steps: usize) -> u64 {
    println!("A: {}", 'A' as u32);
    println!("B: {}", 'B' as u32);
    println!("Z: {}", 'Z' as u32);


    // load data
    let mut lines = input.trim().lines();

    let mut template: Vec<char> = lines.next().unwrap().chars().collect();

    let mut pairs: HashMap<String, char> = HashMap::new();

    for line in lines {

        if line.contains("->") {
            let parts = line.trim().split_once( " -> ").unwrap();

            pairs.insert(parts.0.to_string(), parts.1.chars().next().unwrap());

        }
    }

    println!("{:?}", template);

    println!("{:?}", pairs);

    println!();

    let mut key = String::from("");

    for step in 0..steps {
        println!("{}", step);

        for i in (1..template.len()).rev() {
            key.clear();
            key.push(template[i-1]);
            key.push(template[i]);
    
            // println!("{}", key);
            // println!("{}", pairs[&key]);
    
            if pairs.contains_key(&key) {
                template.insert(i, pairs[&key]);
            }
        }
    }

    // let mut counts: HashMap<char, u64> = HashMap::new();

    let mut counts = vec![0; 26];

    for c in template {
        counts[c as usize] += 1;
    }

    counts.retain(|c| *c > 0);
    counts.sort_unstable();


    // println!("{:?}", template);

    counts[counts.len() - 1] - counts[0]
}

pub fn part_b(input: &str, steps: usize) -> u64 {
    // println!("A: {}", 'A' as u32); // => 65
    // println!("B: {}", 'B' as u32);
    // println!("Z: {}", 'Z' as u32); // => 90


    // load data
    let mut lines = input.trim().lines();

    let template: Vec<char> = lines.next().unwrap().chars().collect();

    let mut pairs: HashMap<String, char> = HashMap::new();

    for line in lines {

        if line.contains("->") {
            let parts = line.trim().split_once( " -> ").unwrap();

            pairs.insert(parts.0.to_string(), parts.1.chars().next().unwrap());

        }
    }

    println!("{:?}", template);

    println!("{:?}", pairs);

    println!();

    // let mut key = String::from("");

    let mut counts = vec![0; 26];
    
    counts[template[template.len() - 1] as usize - 65] += 1;

    let mut memo: HashMap<(char, char, usize), Vec<u64>> = HashMap::new();

    for i in 1..template.len() {
        counts[template[i-1] as usize - 65] += 1;
        increase_char_counts_for_pairs(
            template[i-1],
            template[i],
            steps,
            &pairs,
            &mut memo,
            &mut counts
        );
    }

    // let mut counts: HashMap<char, u64> = HashMap::new();

    counts.retain(|c| *c > 0);
    counts.sort_unstable();


    // println!("{:?}", template);

    println!("{}", counts[counts.len() - 1]);
    println!("{}", counts[0]);

    counts[counts.len() - 1] - counts[0]
}
