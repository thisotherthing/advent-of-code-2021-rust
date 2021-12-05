fn main () {
    dbg!(part_a(include_str!("example.txt")));
    // dbg!(part_a(include_str!("input.txt")));
    // dbg!(part_b(include_str!("example.txt")));
    // dbg!(part_b(include_str!("input.txt")));
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

fn part_b(input: &str) -> i64 {
    let numbers :Vec<&str> = input.trim().split('\n').collect();

    let ox = runFilter(&numbers, true);
    let co = runFilter(&numbers, false);

    println!("{} {}", ox, co);

    ox * co
}

fn runFilter(numbers: &Vec<&str>, _high: bool) -> i64 {

    let num_lines = numbers.len();
    let num_chars = numbers[0].chars().count();

    let mut indices: Vec<usize> = vec![Default::default(); num_lines];

    for i in 0..num_lines { indices[i] = i; }

    let charIndex = 0;

    // while 

    0
}
