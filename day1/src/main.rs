fn main () {
    dbg!(part_a(include_str!("input.txt")));
    dbg!(part_b(include_str!("example.txt")));
    dbg!(part_b(include_str!("input.txt")));
}

pub fn part_a(input: &str) -> i64 {
    let c :Vec<i64> = input.trim().split('\n').map(|l| l.parse::<i64>().unwrap()).collect();

    let mut increases: i64 = 0;

    // for i in 1..c.len() {
    //     if c[i] > c[i-1] {
    //         increases += 1;
    //     }
    // }
    for window in c.windows(2) {
        if window[1] > window[0] {
            increases += 1;
        }
    }

    increases
}

pub fn part_b(input: &str) -> i64 {
    let c :Vec<i64> = input.trim().split('\n').map(|l| l.parse::<i64>().unwrap()).collect();

    let mut increases: i64 = 0;

    for i in 3..c.len() {
        let a = c[i-1] + c[i-2] + c[i-3];
        let b = c[i] + c[i-1] + c[i-2];
        if a < b {
            increases += 1;
        }
    }

    increases
}

// #[cfg(test)]
// mod tests {
//     // #[test]
//     // fn example1() {
//     //     assert_eq!(super::part_a("\n"), 0);
//     // }

//     #[test]
//     fn part_a() {
//         assert_eq!(super::part_a(include_str!("input.txt")), 0);
//     }
// }