fn main () {
    // dbg!(part_a(include_str!("example.txt")));
    dbg!(part_a(include_str!("input.txt")));
    // dbg!(part_b(include_str!("example.txt")));
    dbg!(part_b(include_str!("input.txt")));
}

pub fn part_a(input: &str) -> i64 {
    let numbers :Vec<&str> = input.trim().split('\n').collect();

    let num_chars = numbers[0].chars().count();

    let mut counts: Vec<i32> = vec![Default::default(); num_chars];
    
    for number in numbers {
        for i in 0..num_chars {
            match number.chars().nth(i).unwrap() {
                '0' => { counts[i] -= 1; },
                '1' => { counts[i] += 1; },
                _ => panic!(),
            }
        }
    }
    // print!("{:?}", counts);
    
    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..num_chars {

        let val = 1 << (num_chars - i - 1);

        if counts[i] > 0 {
            gamma = gamma | val;
        } else {
            epsilon |= val;
        }
    }

    // dbg!(gamma);
    // dbg!(epsilon);

    
    gamma * epsilon
}

pub fn part_b(input: &str) -> i64 {
    let numbers :Vec<&str> = input.trim().split('\n').collect();

    let ox = run_filter(&numbers, true);
    let co = run_filter(&numbers, false);

    let ox_v = i64::from_str_radix(numbers[ox], 2).expect("error parsing");
    let co_v = i64::from_str_radix(numbers[co], 2).expect("error parsing");

    // dbg!(ox);

    println!("{} {} {}", ox, numbers[ox], ox_v);
    println!("{} {} {}", co, numbers[co], co_v);

    // ox * co
    ox_v * co_v
}

fn run_filter(numbers: &[&str], high: bool) -> usize {

    let num_lines = numbers.len();
    let num_chars = numbers[0].chars().count();

    let mut indices: Vec<usize> = vec![Default::default(); num_lines];

    for i in 0..num_lines { indices[i] = i; }

    // dbg!(indices);

    let mut char_index = 0;

    for _ in 0..num_chars {
        let mut count = 0;

        for i in 0..indices.len() {
            match numbers[indices[i]].chars().nth(char_index).unwrap() {
                '0' => { count -= 1; },
                '1' => { count += 1; },
                _ => panic!(),
            }
        }

        let char_to_match: char;

        if high {
            if count >= 0 {
                char_to_match = '1';
            } else {
                char_to_match = '0';
            }
        } else {
            if count >= 0 {
                char_to_match = '0';
            } else {
                char_to_match = '1';
            }
        }

        indices.retain(|&idx| {
            return numbers[idx].chars().nth(char_index).unwrap() == char_to_match
        });

        if indices.len() == 1 {
            return indices[0]
        }

        char_index += 1;
    }

    panic!()
}
