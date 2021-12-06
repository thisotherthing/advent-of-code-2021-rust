fn main () {
    // dbg!(part_a(include_str!("example.txt")));
    // dbg!(part_a(include_str!("input.txt")));
    dbg!(part_b(include_str!("example.txt")));
    dbg!(part_b(include_str!("input.txt")));
}

struct Board {
    numbers: Vec<u32>,
    checked: Vec<bool>,
    w: usize,
    h: usize,
    l: usize
}

fn get_board(input: &str) -> Board {
    let lines: Vec<&str> = input.trim().split('\n').collect();

    let h = lines.len();

    let long_input = input.trim().replace('\n', " ");

    let numbers: Vec<u32> =
        long_input.split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let num_numbers = numbers.len();

    let w = num_numbers / lines.len();

    // dbg!(numbers);
    // dbg!((num_numbers / lines.len()) as u32);
    // dbg!(h);

    Board {
        w,
        h,
        numbers,
        checked: vec![false; num_numbers],
        l: w * h,

    }
}

fn check_board_win(board: &Board) -> bool {
    // check down
    for x in 0..board.w {
        if board.checked.iter().skip(x).step_by(board.w).all(|checked| *checked) { return true; }
    }

    // check sideways
    for y in 0..board.h {
        if board.checked.iter().skip(y * board.w).take(board.w).all(|checked| *checked) { return true; }
    }

    false
}

fn update_board(board: &mut Board, number: u32) {
    for i in 0..board.l {
        if !board.checked[i] && board.numbers[i] == number { board.checked[i] = true; }
    }
}

pub fn part_a(input: &str) -> u32 {
    let mut data_chunks= input.trim().split("\n\n");

    let numbers: Vec<u32>= data_chunks.next().unwrap().split(',').map(|i| i.parse::<u32>().unwrap()).collect();

    let mut boards: Vec<Board> = data_chunks.map(get_board).collect();

    // dbg!(numbers);
    // dbg!(boards);

    for number in numbers {
        println!("{}", number);

        for board in &mut boards {
            update_board(board, number);
            if check_board_win(board) {
                let mut result: u32 = 0;

                for i in 0..board.l {
                    if !board.checked[i] { result += board.numbers[i]; }
                }

                return result * number;
            }
        }
    }

    0
}

pub fn part_b(input: &str) -> u32 {
    let mut data_chunks= input.trim().split("\n\n");

    let numbers: Vec<u32>= data_chunks.next().unwrap().split(',').map(|i| i.parse::<u32>().unwrap()).collect();

    let mut boards: Vec<Board> = data_chunks.map(get_board).collect();

    let mut solved: Vec<bool> = vec![Default::default(); boards.len()];

    let mut last_result = 0;

    for number in numbers {
        // println!("{}", number);

        for i in 0..boards.len() {
            update_board(&mut boards[i], number);
            if check_board_win(&boards[i]) && !solved[i] {
                let mut result: u32 = 0;

                for j in 0..boards[i].l {
                    if !boards[i].checked[j] { result += boards[i].numbers[j]; }
                }

                last_result = result * number;

                solved[i] = true;

                println!("solved {}: {}", i, last_result);
            }

            if !solved.contains(&false) { return last_result; }
        }
    }

    last_result
}

// pub fn part_b(input: &str) -> i64 {
//     let numbers :Vec<&str> = input.trim().split('\n').collect();

//     0
// }
