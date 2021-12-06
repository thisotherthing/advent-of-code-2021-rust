use std::{collections::HashMap};

fn main () {
    // dbg!(part_a(include_str!("example.txt")));
    // dbg!(part_a(include_str!("input.txt")));
    dbg!(part_b(include_str!("example.txt")));
    // dbg!(part_b(include_str!("input.txt")));
}

fn add_line_point(positions: &mut HashMap<(u32,u32), u32>, point: (u32, u32)) {
    *positions.entry(point).or_insert(0) += 1;
}

fn add_line_point_i(positions: &mut HashMap<(i32,i32), u32>, point: (i32, i32)) {
    *positions.entry(point).or_insert(0) += 1;
}

pub fn part_a(input: &str) -> u32 {

    let lines: Vec<Vec<u32>> =
        input
        .trim()
        .split('\n')
        .map(|line| line
            .to_string()
            .replace(" ", "")
            .replace("->", ",")
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
        )
        .collect()
        ;

    // dbg!(lines);

    let mut positions: HashMap<(u32,u32), u32> = HashMap::new();

    for line in lines {
        if line.len() == 4 {
            // x line
            if line[1] == line[3] {
                for x in
                line[0].min(line[2])
                ..=
                line[0].max(line[2])
                { add_line_point(&mut positions, (x, line[1])); }
            }

            // y line
            else if line[0] == line[2] {
                for y in
                line[1].min(line[3])
                ..=
                line[1].max(line[3])
                { add_line_point(&mut positions, (line[0], y)); }
            }
        }
    }

    
    for y in 0..12 {
        for x in 0..12 {
            if positions.contains_key(&(x,y)) { print!("{}", positions[&(x,y)]); }
            else { print!(".") }
        }
        println!();
    }
    
    // let mut count = 0;
    // for (_key, value) in positions {
    //     if value > 1 { count += 1; }
    // }

    // count

    positions.values().filter(|c| **c >= 2).count() as u32
}

pub fn part_b(input: &str) -> u32 {

    let lines: Vec<Vec<i32>> =
        input
        .trim()
        .split('\n')
        .map(|line| line
            .to_string()
            .replace(" ", "")
            .replace("->", ",")
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        )
        .collect()
        ;

    // dbg!(lines);

    let mut positions: HashMap<(i32,i32), u32> = HashMap::new();

    for line in lines {
        let x_step:i32 = ((line[2] - line[0]) as i32).signum();
        let y_step:i32 = ((line[3] - line[1]) as i32).signum();

        let mut x = line[0] as i32;
        let mut y = line[1] as i32;

        while x != line[2] || y != line[3] {
            add_line_point_i(&mut positions, (x, y));

            x += x_step;
            y += y_step;
        }

        add_line_point_i(&mut positions, (x, y));
    }

    
    for y in 0..12 {
        for x in 0..12 {
            if positions.contains_key(&(x,y)) { print!("{}", positions[&(x,y)]); }
            else { print!(".") }
        }
        println!();
    }
    
    let mut count = 0;
    for (_key, value) in positions {
        if value > 1 { count += 1; }
    }

    // dbg!(count);
    // dbg!(boards);

    count
}
