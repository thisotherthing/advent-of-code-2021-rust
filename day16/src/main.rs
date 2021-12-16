fn main () {
    // dbg!(part_a("D2FE28"));
    // dbg!(part_a("38006F45291200"));
    // dbg!(part_a("EE00D40C823060"));
    // dbg!(part_a("8A004A801A8002F478"));
    // dbg!(part_a("A0016C880162017C3686B18A3D4780"));
    // dbg!(part_a(include_str!("input.txt")));
    // dbg!( part_b("C200B40A82"));
    assert_eq!(part_b("C200B40A82"), 3);
    assert_eq!(part_b("04005AC33890"), 54);
    assert_eq!(part_b("880086C3E88112"), 7);
    assert_eq!(part_b("CE00C43D881120"), 9);
    assert_eq!(part_b("D8005AC2A8F0"), 1);
    assert_eq!(part_b("F600BC2D8F"), 0);
    assert_eq!(part_b("9C005AC2F8F0"), 0);
    assert_eq!(part_b("9C0141080250320F1802104A08"), 1);
    dbg!(part_b(include_str!("input.txt")));
}

struct Packet {
    ver: u64,
    typ: u64,
    val: u64,
    children: Vec<usize>,
}

impl std::fmt::Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(ver: {}, typ: {}, val : {}, children: {:?})", self.ver, self.typ, self.val, self.children)
    }
}

fn get_number_from_binary_vec(input: &Vec<u8>) -> u64 {
    let mut val = 0u64;

    let l = input.len();

    for (i, v) in input.iter().enumerate() {
        if *v == 1 { val |= 1 << (l - i - 1); }
    }

    val
}

fn get_number_from_input(input: &mut Vec<u8>, l: usize) -> u64 {
    let val = get_number_from_binary_vec(&input[0..l].to_vec());
    input.drain(0..l);

    val
}

fn get_packet(packets: &mut Vec<Packet>, bit_input: &mut Vec<u8>, literal_value_builder: &mut Vec<u8>) -> usize {
    let ver = get_number_from_input(bit_input, 3);
    let typ = get_number_from_input(bit_input, 3);

    let mut val = 0;
    let mut children: Vec<usize> = vec![];

    // parse literal value
    if typ == 4 {
        let mut run = true;

        literal_value_builder.clear();

        while run {
            let is_last = bit_input[0] == 0;
            bit_input.remove(0);

            let l = 4;
            for i in 0..4 {
                literal_value_builder.push(bit_input[i]);
            }
            bit_input.drain(0..l);

            if is_last {
                val = get_number_from_binary_vec(literal_value_builder);
                literal_value_builder.clear();
                run = false;
            }
        }
    } else {
        // operator
        let is_15_or_11 = bit_input[0] == 0;
        bit_input.remove(0);

        if is_15_or_11 {
            // state = ParserState::Operator_15Bit;
            let subpacket_bit_length = get_number_from_input(bit_input, 15) as usize;

            let mut sub = bit_input[0..subpacket_bit_length].to_vec();

            while !sub.is_empty() && sub.len() > 3 {
                children.push(
                    get_packet(packets, &mut sub, literal_value_builder)
                );
            }

            bit_input.drain(0..subpacket_bit_length);
        } else {
            // state = ParserState::Operator1_11Bit;
            let num_of_subpackets = get_number_from_input(bit_input, 11);
            for _ in 0..num_of_subpackets as usize {
                children.push(
                    get_packet(packets, bit_input, literal_value_builder)
                );
            }
        }
    }


    let new = Packet {
        ver,
        typ,
        val,
        children
    };

    packets.push(new);
    packets.len() - 1
}

pub fn part_a(input: &str) -> u64 {

     let mut bit_input: Vec<u8> = vec![];


    for line in input.trim().lines() {
        for c in line.chars() {
            match c {
                '0' => bit_input.extend( [0,0,0,0].iter()),
                '1' => bit_input.extend( [0,0,0,1].iter() ),
                '2' => bit_input.extend( [0,0,1,0].iter() ),
                '3' => bit_input.extend( [0,0,1,1].iter() ),
                '4' => bit_input.extend( [0,1,0,0].iter() ),
                '5' => bit_input.extend( [0,1,0,1].iter() ),
                '6' => bit_input.extend( [0,1,1,0].iter() ),
                '7' => bit_input.extend( [0,1,1,1].iter() ),
                '8' => bit_input.extend( [1,0,0,0].iter() ),
                '9' => bit_input.extend( [1,0,0,1].iter() ),
                'A' => bit_input.extend( [1,0,1,0].iter() ),
                'B' => bit_input.extend( [1,0,1,1].iter() ),
                'C' => bit_input.extend( [1,1,0,0].iter() ),
                'D' => bit_input.extend( [1,1,0,1].iter() ),
                'E' => bit_input.extend( [1,1,1,0].iter() ),
                'F' => bit_input.extend( [1,1,1,1].iter() ),
                _ => {}
            }
        }
    }

    println!("{:?}", bit_input);

    let mut packets: Vec<Packet> = vec![];
    let mut indices: Vec<usize> = vec![];

    let mut literal_value_builder: Vec<u8> = vec![];

    while !bit_input.is_empty() && bit_input.len() > 3 && bit_input.contains(&1) {
        indices.push(
            get_packet(&mut packets, &mut bit_input, &mut literal_value_builder)
        );
    }

    println!("{:?}", packets);

    packets.iter().map(|p| p.ver).sum::<u64>()
}

fn get_value(index: usize, packets: &Vec<Packet>) -> u64 {
    let mut values = vec![];

    for i in 0..packets[index].children.len() {
        values.push(get_value(packets[index].children[i], packets));
    }

    // println!("{:?}", values);
    // println!("{}", packets[index].typ);

    match packets[index].typ {
        0 => { return values.iter().sum(); },
        1 => { return values.iter().product(); },
        2 => { return *values.iter().min().unwrap(); },
        3 => { return *values.iter().max().unwrap(); }
        4 => { packets[index].val }
        5 => {
            if values[0] > values[1] {
                return 1
            }
            0
        }
        6 => {
            if values[0] < values[1] {
                return 1
            }
            0
        }
        7 => {
            if values[0] == values[1] {
                return 1
            }
            0
        }

        _ => { panic!(); }
    }

    // 0
}

pub fn part_b(input: &str) -> u64 {

     let mut bit_input: Vec<u8> = vec![];


    for line in input.trim().lines() {
        for c in line.chars() {
            match c {
                '0' => bit_input.extend( [0,0,0,0].iter()),
                '1' => bit_input.extend( [0,0,0,1].iter() ),
                '2' => bit_input.extend( [0,0,1,0].iter() ),
                '3' => bit_input.extend( [0,0,1,1].iter() ),
                '4' => bit_input.extend( [0,1,0,0].iter() ),
                '5' => bit_input.extend( [0,1,0,1].iter() ),
                '6' => bit_input.extend( [0,1,1,0].iter() ),
                '7' => bit_input.extend( [0,1,1,1].iter() ),
                '8' => bit_input.extend( [1,0,0,0].iter() ),
                '9' => bit_input.extend( [1,0,0,1].iter() ),
                'A' => bit_input.extend( [1,0,1,0].iter() ),
                'B' => bit_input.extend( [1,0,1,1].iter() ),
                'C' => bit_input.extend( [1,1,0,0].iter() ),
                'D' => bit_input.extend( [1,1,0,1].iter() ),
                'E' => bit_input.extend( [1,1,1,0].iter() ),
                'F' => bit_input.extend( [1,1,1,1].iter() ),
                _ => {}
            }
        }
    }

    // println!("{:?}", bit_input);

    let mut packets: Vec<Packet> = vec![];
    let mut indices: Vec<usize> = vec![];

    let mut literal_value_builder: Vec<u8> = vec![];

    while !bit_input.is_empty() && bit_input.len() > 3 && bit_input.contains(&1) {
        indices.push(
            get_packet(&mut packets, &mut bit_input, &mut literal_value_builder)
        );
    }

    // println!("{:?}", packets);

    get_value(indices[0], &packets)
}
