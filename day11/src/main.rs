use std::ops::Rem;
fn main () {
    // dbg!(part_a(include_str!("example_short.txt"), 2));
    // dbg!(part_a(include_str!("example.txt"), 100));
    dbg!(part_a(include_str!("input.txt"), 100));
    // dbg!(part_b(include_str!("example.txt")));
    dbg!(part_b(include_str!("input.txt")));
}

fn run_flash(squids: &mut Vec<u32>, idx: usize, width: usize) {

    squids[idx] += 1000;

    let x = idx.rem(width) as i64;
    let y = (idx / width) as i64;

    let num_squids = squids.len() as i64;
    let w = width as i64;

    let mut tmp_idx;

    for mx in -1..=1 {
        for my in -1..=1 {
            if x + mx >= 0 && x + mx < w {
                tmp_idx = (y + my) * w + x + mx;

                if tmp_idx >= 0 && tmp_idx < num_squids {
                    squids[tmp_idx as usize] += 1;
                }
            }
        }
    }
}

fn run_flash_check(squids: &mut Vec<u32>, width: usize) -> bool {
    let mut flash = false;

    for i in 0..squids.len() {
        if squids[i] > 9 && squids[i] < 1000 {
            run_flash(squids, i, width);
            flash = true;
        }
    }

    flash
}

fn print_state(squids: &Vec<u32>, width: usize) {
    // println!("{:?}",
    //     squids.into_iter().map(|n| n.to_string()).collect::<String>()
    // );

    for (n, s) in squids.iter().enumerate() {
        if n.rem(width) == 0 {
           println!();
        }
        print!("{}", s);
     }
}

pub fn part_a(input: &str, num_days: u32) -> u64 {
    let mut num_flashes: u64 = 0;

    let mut lines = input.trim().lines();
    let line_length = lines.next().unwrap().trim().chars().count();

    let mut squids: Vec<u32> = vec![0; (lines.count() + 1) * line_length];

    // load data
    {
        let mut idx: usize = 0;
        for line in input.trim().lines() {
            for c in line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
            {
                squids[idx] = c;
                idx += 1;
            }
        }
    }

    print_state(&squids, line_length);
    println!();

    for _ in 0..num_days {
        // add one
        for i in 0..squids.len() {
            squids[i] += 1;
        }

        // run flashes
        {
            let mut rerun = true;
    
            while rerun { rerun = run_flash_check(&mut squids, line_length); }
        }
    
        // check flash count and reset
        {
            for i in 0..squids.len() {
                if squids[i] > 9 {
                    num_flashes += 1;
                    squids[i] = 0;
                }
            }
        }
        
        print_state(&squids, line_length);
        println!();
    }


    num_flashes
}

pub fn part_b(input: &str) -> u64 {
    let mut num_days: u64 = 1;

    let mut lines = input.trim().lines();
    let line_length = lines.next().unwrap().trim().chars().count();

    let mut squids: Vec<u32> = vec![0; (lines.count() + 1) * line_length];
    let num_squids = squids.len();

    // load data
    {
        let mut idx: usize = 0;
        for line in input.trim().lines() {
            for c in line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
            {
                squids[idx] = c;
                idx += 1;
            }
        }
    }

    // print_state(&squids, line_length);

     loop {
        // add one
        for i in 0..num_squids {
            squids[i] += 1;
        }

        // run flashes
        {
            let mut rerun = true;
    
            while rerun { rerun = run_flash_check(&mut squids, line_length); }
        }
    
        // check flash count and reset
        {
            let mut current_flashes = 0;

            for i in 0..num_squids {
                if squids[i] > 9 {
                    current_flashes += 1;
                    squids[i] = 0;
                }
            }

            if current_flashes == num_squids { return num_days; }
        }
        
        // print_state(&squids, line_length);

        num_days += 1;
    }
}
