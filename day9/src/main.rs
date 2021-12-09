use std::collections::HashSet;

fn main () {
    // dbg!(part_a(include_str!("example.txt")));
    // dbg!(part_a(include_str!("input.txt")));
    dbg!(part_b(include_str!("example.txt")));
    dbg!(part_b(include_str!("input.txt")));
}

pub fn part_a(input: &str) -> u32 {
    let heights: Vec<Vec<u32>> =
        input
            .trim()
            .split('\n')
            .map(
                |line| line.trim().split("")
                .filter(|l| !l.is_empty())
                .map(|c|  { 
                    // println!("{}", c.trim());
                    c.trim().parse::<u32>().unwrap()
                    // 0
                })
                .collect()
            ).collect();

    let num_lines = heights.len();
    let line_length = heights[0].len();

    let mut risk = 0;

    let mut low;
    let mut h;
    for x in 0..line_length {
        for y in 0..num_lines {
            low = true;
            h = heights[y][x];

            // left
            if x > 0 { low = low && h < heights[y][x-1]; }

            // right
            if x < line_length - 1 { low = low && h < heights[y][x+1]; }

            // top
            if y > 0 { low = low && h < heights[y-1][x]; }

            // bottom
            if y < num_lines - 1 { low = low && h < heights[y+1][x]; }

            if low {
                println!("{}", h);
                risk += h + 1;
            }
        }
    }

    risk
}

fn grow_basin(checked: &mut HashSet<(usize, usize)>, heights: &[Vec<u32>], max_x: usize, max_y: usize, x: usize, y: usize) {
    if checked.contains(&(x, y)) { return; }

    if heights[y][x] < 9 {
        checked.insert((x, y));
    } else {
        return;
    }

    // top
    if y > 0 { grow_basin(checked, heights, max_x, max_y, x, y - 1); }

    // bottom
    if y < max_y { grow_basin(checked, heights, max_x, max_y, x, y + 1); }

    // left
    if x > 0 { grow_basin(checked, heights, max_x, max_y, x - 1, y ); }

    // right
    if x < max_x { grow_basin(checked, heights, max_x, max_y, x + 1, y); }
}

fn get_basin_size(heights: &[Vec<u32>], x: usize, y: usize) -> usize {

    let mut checked : HashSet<(usize, usize)> = HashSet::new();

    grow_basin(&mut checked, heights, heights[0].len() - 1, heights.len() -1, x, y);

    checked.len()
}

pub fn part_b(input: &str) -> usize {
    let heights: Vec<Vec<u32>> =
        input
            .trim()
            .split('\n')
            .map(
                |line| line.trim().split("")
                .filter(|l| !l.is_empty())
                .map(|c|  { 
                    // println!("{}", c.trim());
                    c.trim().parse::<u32>().unwrap()
                    // 0
                })
                .collect()
            ).collect();

    let num_lines = heights.len();
    let line_length = heights[0].len();

    let mut basins: Vec<usize> = vec![];

    let mut low;
    let mut h;
    for x in 0..line_length {
        for y in 0..num_lines {
            low = true;
            h = heights[y][x];

            // left
            if x > 0 { low = low && h < heights[y][x-1]; }

            // right
            if x < line_length - 1 { low = low && h < heights[y][x+1]; }

            // top
            if y > 0 { low = low && h < heights[y-1][x]; }

            // bottom
            if y < num_lines - 1 { low = low && h < heights[y+1][x]; }

            if low {
                let size = get_basin_size(&heights, x, y);

                if basins.len() < 3 {
                    basins.push(size);
                } else {
                    basins.sort_unstable();

                    if size > basins[0] {
                        basins.remove(0);
                        basins.push(size);
                    }
                }
            }
        }
    }

    basins.iter().product()
}
