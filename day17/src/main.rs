use regex::Regex;

fn main () {
    // dbg!(part_a(include_str!("example.txt")));
    // dbg!(part_a(include_str!("input.txt")));
    dbg!(part_b(include_str!("example.txt")));
    dbg!(part_b(include_str!("input.txt")));
}

struct Bounds {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

fn shoot(vel_x: i64, vel_y: i64, bounds: &Bounds) -> i64 {
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    let mut vel_x = vel_x;
    let mut vel_y = vel_y;

    let mut local_highest: i64 = i64::MIN;

    while
        // x.signum() == bounds.max_x.signum() &&
        x <= bounds.max_x &&
        y >= bounds.min_y
    {
        x += vel_x;
        y += vel_y;

        local_highest = local_highest.max(y);

        if  
            x >= bounds.min_x &&
            x <= bounds.max_x &&
            y >= bounds.min_y &&
            y <= bounds.max_y
        {
            return local_highest
        }

        vel_y -= 1;

        if vel_x > 0 {
            vel_x -= 1;
        } else {
            // also stop if there's no more x velocity and we missed the window
            if x < bounds.min_x || x > bounds.max_x { break }
        }
    }
    i64::MIN
}
fn shoot_b(vel_x: i64, vel_y: i64, bounds: &Bounds) -> bool {
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    let mut vel_x = vel_x;
    let mut vel_y = vel_y;

    while
        // x.signum() == bounds.max_x.signum() &&
        x <= bounds.max_x &&
        y >= bounds.min_y
    {
        x += vel_x;
        y += vel_y;


        if  
            x >= bounds.min_x &&
            x <= bounds.max_x &&
            y >= bounds.min_y &&
            y <= bounds.max_y
        {
            return true
        }

        vel_y -= 1;

        if vel_x > 0 {
            vel_x -= 1;
        } else {
            // also stop if there's no more x velocity and we missed the window
            if x < bounds.min_x || x > bounds.max_x { break }
        }
    }
    
    false
}

pub fn part_a(input: &str) -> i64 {

    let re = Regex::new(r"(-?\d+)").unwrap();

    let mut cap =  re.captures_iter(input);

    let bounds = Bounds {
        min_x: cap.next().unwrap()[1].parse::<i64>().unwrap(),
        max_x: cap.next().unwrap()[1].parse::<i64>().unwrap(),
        min_y: cap.next().unwrap()[1].parse::<i64>().unwrap(),
        max_y: cap.next().unwrap()[1].parse::<i64>().unwrap(),
    };

    let mut highest = i64::MIN;

    for x in 1..=(bounds.min_x / 2) {
        for y in -10..200 {
            highest = highest.max(shoot(x, y, &bounds));
        }
    }

    println!("{} {}..{} {}..{}", input, bounds.min_x, bounds.max_x, bounds.min_y, bounds.max_y);

    highest
}
pub fn part_b(input: &str) -> i64 {

    let re = Regex::new(r"(-?\d+)").unwrap();

    let mut cap =  re.captures_iter(input);

    let bounds = Bounds {
        min_x: cap.next().unwrap()[1].parse::<i64>().unwrap(),
        max_x: cap.next().unwrap()[1].parse::<i64>().unwrap(),
        min_y: cap.next().unwrap()[1].parse::<i64>().unwrap(),
        max_y: cap.next().unwrap()[1].parse::<i64>().unwrap(),
    };

    // let mut highest = i64::MIN;
    let mut count = 0;

    for x in 1..=bounds.max_x {
        for y in -200..200 {
            if shoot_b(x, y, &bounds) {
                count += 1;
            }
        }
    }

    println!("{} {}..{} {}..{}", input, bounds.min_x, bounds.max_x, bounds.min_y, bounds.max_y);

    count
}
