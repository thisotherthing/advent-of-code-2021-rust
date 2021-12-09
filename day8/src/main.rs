use std::{collections::{HashMap}, borrow::{Borrow}};

fn main () {
    // dbg!(part_a(include_str!("example.txt")));
    // dbg!(part_a(include_str!("input.txt")));
    // dbg!(part_b(include_str!("example_short.txt")));
    dbg!(part_b(include_str!("example.txt")));
    dbg!(part_b(include_str!("input.txt")));
}




pub fn part_a(input: &str) -> u32 {

    let mut _counts: HashMap<char, u32> = HashMap::new();

    let mut counter = 0;

    for line in input.trim().split('\n').collect::<Vec<&str>>() {
        // println!("{}", line);

        let parts = line.split('|').map(|s: &str| s.trim()).collect::<Vec<&str>>();

        // println!("{}", parts[0]);
        // println!("{}", parts[1]);

        let second: Vec<&str> = parts[1].split_whitespace().collect();
        for s in second {
            if s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7 { counter += 1; }
            // *count.entry(parse_segments_string(s)).or_insert(0) += 1;
        }
    }

    // counter += *count.entry('1').or_default();
    // counter += *count.entry('4').or_default();
    // counter += *count.entry('7').or_default();
    // counter += *count.entry('8').or_default();

    counter

    // let test_input = "cf";

    // dbg!(parse_segments_string("abcefg", '0'));
    // dbg!(parse_segments_string("cf"));
    // dbg!(parse_segments_string("acdeg", '2'));
    // dbg!(parse_segments_string("acdfg", '3'));
    // dbg!(parse_segments_string("bcdf", '4'));
    // dbg!(parse_segments_string("abdfg", '5'));
    // dbg!(parse_segments_string("abdefg", '6'));
    // dbg!(parse_segments_string("acf", '7'));
    // dbg!(parse_segments_string("abcdefg", '8'));
    // dbg!(parse_segments_string("abcdfg", '9'));
}

fn _parse_segments_string(digit: &str, map: &HashMap<char,Vec<char>>) -> char {
    let mut val = 0u32;

    if digit.contains(map[&'a'][0]) { val |= 1 << 0 }
    if digit.contains(map[&'b'][0]) { val |= 1 << 1 }
    if digit.contains(map[&'c'][0]) { val |= 1 << 2 }
    if digit.contains(map[&'d'][0]) { val |= 1 << 3 }
    if digit.contains(map[&'e'][0]) { val |= 1 << 4 }
    if digit.contains(map[&'f'][0]) { val |= 1 << 5 }
    if digit.contains(map[&'g'][0]) { val |= 1 << 6 }

    let mapping = HashMap::from([
        (0b01110111, '0'),
        (0b00100100, '1'),
        (0b01011101, '2'),
        (0b01101101, '3'),
        (0b00101110, '4'),
        (0b01101011, '5'),
        (0b01111011, '6'),
        (0b00100101, '7'),
        (0b01111111, '8'),
        (0b01101111, '9'),
    ]);

    let mut str = '_';

    // println!("{:#010b} {}", val, digit);

    if mapping.contains_key(&val) { str = mapping[&val]; }

    str
}

fn update_map(data: Vec<&str>, map: &mut HashMap<char,Vec<char>>) {

    // println!("{:?}", data);

    for s in data {
        // println!("string {}", s);
        let chars: Vec<char> = s.trim().chars().collect();
        let length = chars.len();



        let mut missing = vec!['a','b','c','d','e','f','g'];
        // println!("missing before {:?}", missing);
        for c in s.chars() { missing.retain(|sc| *sc != c); }
        // println!("missing after {:?}", missing);


        // for c in s.chars() {
        // println!("char {}", c);

        match length {
            // // 8
            // 7 => { map.get_mut(&'d').unwrap().retain(|f| *f != missing[0]) }

            // 7
            3 => {
                // println!("{:?}, {:?}", chars, map.get_mut(&'a'));
                map.get_mut(&'a').unwrap().retain(|f| chars.contains(f));
                map.get_mut(&'c').unwrap().retain(|f| chars.contains(f));
                map.get_mut(&'f').unwrap().retain(|f| chars.contains(f));

                map.get_mut(&'b').unwrap().retain(|f| !chars.contains(f));
                map.get_mut(&'d').unwrap().retain(|f| !chars.contains(f));
                map.get_mut(&'e').unwrap().retain(|f| !chars.contains(f));
                map.get_mut(&'g').unwrap().retain(|f| !chars.contains(f));
            }

            // 5 - 2, 3, 5
            5 => {
                
                map.get_mut(&'a').unwrap().retain(|f| chars.contains(f));
                map.get_mut(&'d').unwrap().retain(|f| chars.contains(f));
                map.get_mut(&'g').unwrap().retain(|f| chars.contains(f));
            }

            // 6 - 0, 6, 9
            6 => {
                
                map.get_mut(&'a').unwrap().retain(|f| chars.contains(f));
                map.get_mut(&'b').unwrap().retain(|f| chars.contains(f));
                map.get_mut(&'f').unwrap().retain(|f| chars.contains(f));
                map.get_mut(&'g').unwrap().retain(|f| chars.contains(f));
            }

            // 4
            4 => {
                // println!("{:>?}", chars);
                // println!("before {:?}", map.get_mut(&'a'));
                map.get_mut(&'b').unwrap().retain(|f| chars.contains(f));
                map.get_mut(&'c').unwrap().retain(|f| chars.contains(f));
                map.get_mut(&'d').unwrap().retain(|f| chars.contains(f));
                map.get_mut(&'f').unwrap().retain(|f| chars.contains(f));

                map.get_mut(&'a').unwrap().retain(|f| !chars.contains(f));
                map.get_mut(&'e').unwrap().retain(|f| !chars.contains(f));
                map.get_mut(&'g').unwrap().retain(|f| !chars.contains(f));
                // println!("after {:?}", map.get_mut(&'a'));
            }

            // 1
            2 => {
                map.get_mut(&'c').unwrap().retain(|f| chars.contains(f));
                map.get_mut(&'f').unwrap().retain(|f| chars.contains(f));

                map.get_mut(&'a').unwrap().retain(|f| !chars.contains(f));
                map.get_mut(&'b').unwrap().retain(|f| !chars.contains(f));
                map.get_mut(&'d').unwrap().retain(|f| !chars.contains(f));
                map.get_mut(&'e').unwrap().retain(|f| !chars.contains(f));
                map.get_mut(&'g').unwrap().retain(|f| !chars.contains(f));
            }

            _ => {}
        }
    }

    for _ in 0..5 {
        // collect already picked segments
        let mut picked: Vec<char> = vec![];
        for chars in map.values() { if chars.len() == 1 { picked.push(chars[0]) } }

        // if all are picked, the map is finished
        if picked.len() == map.keys().len() { return }

        // filter out picked options
        for chars in map.values_mut() { if chars.len() > 1 {
            chars.retain(|f| !picked.contains(f));
        } }
    }
}

// 0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....

//  5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

// 1, 4, 7, 8

// 5 - 2, 3, 5
// 6 - 0, 6, 9

pub fn part_b(input: &str) -> u64 {

    let mut counter = 0u64;

    for line in input.trim().split('\n').collect::<Vec<&str>>() {
        // println!("{}", line);

        let mut map: HashMap<char, Vec<char>> = HashMap::from([
            ('a', vec!['a','b','c','d','e','f','g']),
            ('b', vec!['a','b','c','d','e','f','g']),
            ('c', vec!['a','b','c','d','e','f','g']),
            ('d', vec!['a','b','c','d','e','f','g']),
            ('e', vec!['a','b','c','d','e','f','g']),
            ('f', vec!['a','b','c','d','e','f','g']),
            ('g', vec!['a','b','c','d','e','f','g']),
        ]);

        update_map( line.replace('|', " ").split_whitespace().map(|s| s.trim()).collect(), &mut map);
        // update_map(second, &mut map);

        // for (key, val) in &map {
        //     println!("{}: {:?}", key, val);
        // }

        // println!("{}", parts[0]);
        // println!("{}", parts[1]);
        
        let mut result: Vec<char> = vec![];

        let second_part: Vec<&str> = line.split('|').collect::<Vec<&str>>()[1].split_whitespace().map(|s| s.trim()).collect();

        for s in second_part {
            // if s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7 { counter += 1; }

            result.push(
                *_parse_segments_string(s, &map).borrow()
            );

            // *count.entry(parse_segments_string(s)).or_insert(0) += 1;
        }

        println!("{:?}", result);

        counter += result.iter().collect::<String>().parse::<u64>().unwrap();
    }

    // counter += *count.entry('1').or_default();
    // counter += *count.entry('4').or_default();
    // counter += *count.entry('7').or_default();
    // counter += *count.entry('8').or_default();

    // let test_input = "cf";

    // dbg!(parse_segments_string("abcefg", '0'));
    // dbg!(parse_segments_string("cf"));
    // dbg!(parse_segments_string("acdeg", '2'));
    // dbg!(parse_segments_string("acdfg", '3'));
    // dbg!(parse_segments_string("bcdf", '4'));
    // dbg!(parse_segments_string("abdfg", '5'));
    // dbg!(parse_segments_string("abdefg", '6'));
    // dbg!(parse_segments_string("acf", '7'));
    // dbg!(parse_segments_string("abcdefg", '8'));
    // dbg!(parse_segments_string("abcdfg", '9'));

    counter
}
