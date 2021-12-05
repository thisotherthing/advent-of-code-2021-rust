fn main () {
    dbg!(part_a(include_str!("example.txt")));
    dbg!(part_a(include_str!("input.txt")));
    dbg!(part_b(include_str!("example.txt")));
    dbg!(part_b(include_str!("input.txt")));
}

fn part_a(input: &str) -> i64 {
    let mut depth: i64 = 0;
    let mut x: i64 = 0;

    let mut dir: &str;
    let mut amount: i64;

    for line in input.trim().split('\n') {
        let mut parts = line.split(' ');

        dir = parts.next().unwrap();
        amount = parts.next().unwrap().parse::<i64>().unwrap();

        match dir {
            "forward" => { x += amount; }
            "down" => { depth += amount; }
            "up" => { depth -= amount; }
            _ => panic!(),
        }

        // dbg!(dir);
        // dbg!(amount);
    }

    depth * x
}

fn part_b(input: &str) -> i64 {
    let mut depth: i64 = 0;
    let mut x: i64 = 0;
    let mut aim: i64 = 0;

    let mut dir: &str;
    let mut amount: i64;

    for line in input.trim().split('\n') {
        let mut parts = line.split(' ');

        dir = parts.next().unwrap();
        amount = parts.next().unwrap().parse::<i64>().unwrap();

        match dir {
            "down" => { aim += amount; }
            "up" => { aim -= amount; }
            "forward" => {
                x += amount;
                depth += aim * amount;
            }
            _ => panic!(),
        }
    }

    depth * x
}

// pub fn part_b(input: &str) -> i64 {
//     let c :Vec<i64> = input.trim().split('\n').map(|l| l.parse::<i64>().unwrap()).collect();

//     let mut increases: i64 = 0;

//     for i in 3..c.len() {
//         let a = c[i-1] + c[i-2] + c[i-3];
//         let b = c[i] + c[i-1] + c[i-2];
//         if a < b {
//             increases += 1;
//         }
//     }

//     increases
// }
