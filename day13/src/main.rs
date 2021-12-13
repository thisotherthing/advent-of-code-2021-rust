use std::collections::HashSet;

fn main () {
    // dbg!(part_a(include_str!("example.txt")));
    dbg!(part_a(include_str!("input.txt")));
    // dbg!(part_b(include_str!("example.txt")));
    // dbg!(part_b(include_str!("input.txt")));
}

fn show(dots: &HashSet<(u64, u64)>, w: u64, h: u64) {
    for y in 0..=h {
        for x in 0..=w {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    println!();
}

fn fold_y(fold: u64, dots: &mut HashSet<(u64, u64)>, w: &mut u64, h: &mut u64) {
    // println!("fold y");

    let overlap_dist = (fold).min(*h - fold);

    // fold
    for y in 1..=overlap_dist {
        for x in 0..=(*w) {
            let target = (x, fold-y);
            let src = &(x, fold+y);
            if dots.contains(src) && !dots.contains(&target) {
                dots.insert(target);
            }
        }
    }

    // cleanup
    for y in fold..=(*h) {
        for x in 0..=(*w) {
            if dots.contains(&(x, y)) {
                dots.remove(&(x, y));
            }
        }
    }

    *h = (*h).min(fold - 1)
}
fn fold_x(fold: u64, dots: &mut HashSet<(u64, u64)>, w: &mut u64, h: &mut u64) {
    // println!("fold x");

    let overlap_dist = (fold).min(*w - fold);

    // fold
    for y in 0..=(*h) {
        for x in 1..=overlap_dist {
            let target = (fold-x, y);
            let src = &(fold+x, y);
            if dots.contains(src) && !dots.contains(&target) {
                dots.insert(target);
            }
        }
    }

    // cleanup
    for y in 0..=(*h) {
        for x in fold..=(*w) {
            if dots.contains(&(x, y)) {
                dots.remove(&(x, y));
            }
        }
    }

    *w = (*w).min(fold - 1)
}

pub fn part_a(input: &str) -> usize {
    let mut dots: HashSet<(u64, u64)> = HashSet::new();

    let mut max_x = u64::MIN;
    let mut max_y = u64::MIN;

    // load data
    {
        for line in input.trim().lines() {
            if line.contains(',') {
                let coord = line.split_once(',').unwrap();
                let x = coord.0.parse::<u64>().unwrap();
                let y = coord.1.parse::<u64>().unwrap();

                max_x = max_x.max(x);
                max_y = max_y.max(y);

                dots.insert((x, y));
            }

            // print starte state
            // if line.is_empty() {
            //     show(&dots, max_x, max_y);
            // }
            
            if line.contains("fold along") {
                let reduced = line.replace("fold along", "");
                let parts = reduced.trim().split_once('=').unwrap();
                let val = parts.1.parse::<u64>().unwrap();

                if parts.0 == "x" {
                    fold_x(val, &mut dots, &mut max_x, &mut max_y);
                } else {
                    fold_y(val, &mut dots, &mut max_x, &mut max_y);
                }

                // show(&dots, max_x, max_y);

                // return dots.len()
            }
        }
    }

    // let start: usize = nodes_ids["start"];
    // let end: usize = nodes_ids["end"];

    // println!("{:?}", nodes_ids);

    // for i in 0..nodes.len() {
    //     println!("{}", nodes[i].name);
    //     for j in 0..nodes[i].connections.len() {
    //         print!(",{}", nodes[nodes[i].connections[j]].name);
    //     }
    //     println!()
    // }

    show(&dots, max_x, max_y);
    dots.len()
}
