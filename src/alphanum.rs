const ALPHANUMS: [u8; 45] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
    b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V',
    b'W', b'X', b'Y', b'Z', b' ', b'$', b'%', b'*', b'+', b'-', b'.', b'/', b':',
];

const REVERSE_ALPHANUMS: [u16; 128] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    36, 0, 0, 0, 37, 38, 0, 0, 0, 0, 39, 40, 0, 41, 42, 43, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 44, 0, 0,
    0, 0, 0, 0, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 32, 33, 34, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

fn verify(to_encode: &[u8]) -> bool {
    for c in to_encode {
        if !ALPHANUMS.contains(&c) {
            return false;
        }
    }

    return true;
}

fn format_alphanum(b: u16, version: usize) -> String {
    return match version {
        1..=9 => format!("{:09b}", b),
        10..=26 => format!("{:011b}", b),
        27..=40 => format!("{:013b}", b),
        _ => panic!("Version should not be {}", version),
    };
}

pub fn create_pairs(from: &[u8], version: usize) -> String {
    assert!(verify(from));

    let mut res = String::new();

    let tmp = from
        .chunks_exact(2)
        .map(|a| REVERSE_ALPHANUMS[a[0] as usize] * 45 + REVERSE_ALPHANUMS[a[1] as usize]);
    for slice in tmp {
        res.push_str(&format_alphanum(slice, version));
    }

    if from.len() % 2 != 0 {
        res.push_str(&format_alphanum(
            REVERSE_ALPHANUMS[*from.last().unwrap() as usize],
            version,
        ));
    }

    return res;
}