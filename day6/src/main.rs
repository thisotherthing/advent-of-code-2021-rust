use std::collections::HashMap;

fn main () {
    dbg!(part_a(include_str!("example.txt"), 30));
    // dbg!(part_a(include_str!("input.txt")));
    dbg!(part_b(include_str!("example.txt"), 30));
    // dbg!(part_b(include_str!("input.txt"), 256));
}

pub fn part_a(input: &str, days: u32) -> usize {

    let mut fish: Vec<u32> =
        input
        .trim()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
        ;

    for _ in 0..days {
        for i in 0..fish.len() {
            if fish[i] ==  0 {
                fish.push(8);
                fish[i] = 6;
            } else {
                fish[i] -= 1;
            }
        }

        // println!("{} {}", d, fish.len());
    }

    // dbg!(count);
    // dbg!(boards);

    fish.len()
}

fn get_num_fish(state: u32, days: u32, memo_state_days_to_count: &mut HashMap<(u32, u32), usize>) ->  usize {
    if memo_state_days_to_count.contains_key(&(state, days)) {
        return memo_state_days_to_count[&(state, days)];
    }

    let mut count = 1;
    let mut tmp_state = state;

    for d in 0..days {
        if tmp_state == 0 {
            count += get_num_fish(8, days - d - 1, memo_state_days_to_count);
            tmp_state = 6;
        } else {
            tmp_state -= 1;
        }
    }

    memo_state_days_to_count.insert((state, days), count);

    count
}

pub fn part_b(input: &str, days: u32) -> usize {
    let fish: Vec<u32> =
        input
        .trim()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
        ;

    let mut memo_state_days_to_count: HashMap<(u32, u32), usize> = HashMap::new();

    let mut count = 0;

    for f in fish {
        count += get_num_fish(f, days, &mut memo_state_days_to_count);
    }

    count
}
