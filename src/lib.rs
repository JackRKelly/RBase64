use std::collections::HashMap;

const CHARACTER_INDEX: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

pub fn main() {
    let decode = "TWF=";
    dbg!(decode_base64(decode));
    let encode = b"Man";
    dbg!(encode_base64(encode));
}

pub fn first_char(a: u8) -> u8 {
    (a & 0b1111_1100) >> 2
}

pub fn second_char(a: u8, b: u8) -> u8 {
    ((a & 0b0000_0011) << 4) | ((b & 0b1111_0000) >> 4)
}

pub fn third_char(a: u8, b: u8) -> u8 {
    ((a & 0b0000_1111) << 2) | ((b & 0b1100_0000) >> 6)
}

pub fn fourth_char(b: u8) -> u8 {
    b & 0b0011_1111
}

pub fn decode_base64(base64: &str) -> Vec<u8> {
    let mut final_bytes: Vec<u8> = Vec::with_capacity(base64.len() / 4 * 3);

    let mut letter_index = HashMap::new();
    letter_index.insert('A', 0);
    letter_index.insert('B', 1);
    letter_index.insert('C', 2);
    letter_index.insert('D', 3);
    letter_index.insert('E', 4);
    letter_index.insert('F', 5);
    letter_index.insert('G', 6);
    letter_index.insert('H', 7);
    letter_index.insert('I', 8);
    letter_index.insert('J', 9);
    letter_index.insert('K', 10);
    letter_index.insert('L', 11);
    letter_index.insert('M', 12);
    letter_index.insert('N', 13);
    letter_index.insert('O', 14);
    letter_index.insert('P', 15);
    letter_index.insert('Q', 16);
    letter_index.insert('R', 17);
    letter_index.insert('S', 18);
    letter_index.insert('T', 19);
    letter_index.insert('U', 20);
    letter_index.insert('V', 21);
    letter_index.insert('W', 22);
    letter_index.insert('X', 23);
    letter_index.insert('Y', 24);
    letter_index.insert('Z', 25);
    letter_index.insert('a', 26);
    letter_index.insert('b', 27);
    letter_index.insert('c', 28);
    letter_index.insert('d', 29);
    letter_index.insert('e', 30);
    letter_index.insert('f', 31);
    letter_index.insert('g', 32);
    letter_index.insert('h', 33);
    letter_index.insert('i', 34);
    letter_index.insert('j', 35);
    letter_index.insert('k', 36);
    letter_index.insert('l', 37);
    letter_index.insert('m', 38);
    letter_index.insert('n', 39);
    letter_index.insert('o', 40);
    letter_index.insert('p', 41);
    letter_index.insert('q', 42);
    letter_index.insert('r', 43);
    letter_index.insert('s', 44);
    letter_index.insert('t', 45);
    letter_index.insert('u', 46);
    letter_index.insert('v', 47);
    letter_index.insert('w', 48);
    letter_index.insert('x', 49);
    letter_index.insert('y', 50);
    letter_index.insert('z', 51);
    letter_index.insert('0', 52);
    letter_index.insert('1', 53);
    letter_index.insert('2', 54);
    letter_index.insert('3', 55);
    letter_index.insert('4', 56);
    letter_index.insert('5', 57);
    letter_index.insert('6', 58);
    letter_index.insert('7', 59);
    letter_index.insert('8', 60);
    letter_index.insert('9', 61);
    letter_index.insert('+', 62);
    letter_index.insert('/', 63);
    letter_index.insert('=', 64);

    assert!(base64.len() % 4 == 0);

    let byte = base64
        .chars()
        .map(|c| *letter_index.get(&c).unwrap())
        .collect::<Vec<u8>>();

    let chunks = byte.chunks_exact(4);

    for chunk in chunks {
        let mut start: u32 = ((chunk[0] as u32) << 18) | ((chunk[1] as u32) << 12);

        if chunk[2] & chunk[3] == 64 {
            final_bytes.extend_from_slice(&start.to_be_bytes()[1..2]);
        } else if chunk[3] == 64 {
            start |= (chunk[2] as u32) << 6;
            final_bytes.extend_from_slice(&start.to_be_bytes()[1..3]);
        } else {
            start |= (chunk[2] as u32) << 6 | (chunk[3] as u32);
            final_bytes.extend_from_slice(&start.to_be_bytes()[1..4]);
        }
    }

    final_bytes
}

pub fn encode_base64(bytes: &[u8]) -> String {
    let mut final_string: String = String::with_capacity((bytes.len() as f64 * 1.25) as usize);

    let mut chunks = bytes.chunks_exact(3);
    while let Some(chunk) = chunks.next() {
        final_string.push(CHARACTER_INDEX[usize::from(first_char(chunk[0]))]);
        final_string.push(CHARACTER_INDEX[usize::from(second_char(chunk[0], chunk[1]))]);
        final_string.push(CHARACTER_INDEX[usize::from(third_char(chunk[1], chunk[2]))]);
        final_string.push(CHARACTER_INDEX[usize::from(fourth_char(chunk[2]))]);
    }

    let remainder = chunks.remainder();

    match remainder.len() {
        0 => return final_string,
        1 => {
            final_string.push(CHARACTER_INDEX[usize::from(first_char(remainder[0]))]);
            final_string.push(CHARACTER_INDEX[usize::from(second_char(remainder[0], 0))]);
            final_string.push_str("==");
        }
        2 => {
            final_string.push(CHARACTER_INDEX[usize::from(first_char(remainder[0]))]);
            final_string
                .push(CHARACTER_INDEX[usize::from(second_char(remainder[0], remainder[1]))]);
            final_string.push(CHARACTER_INDEX[usize::from(third_char(remainder[1], 0))]);
            final_string.push('=');
        }
        _ => unreachable!(),
    }

    if !remainder.is_empty() {}

    final_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_0_padding() {
        assert_eq!(decode_base64("TWFu"), [77, 97, 110]);
    }

    #[test]
    fn test_decode_1_padding() {
        assert_eq!(decode_base64("TWF="), [77, 97]);
    }

    #[test]
    fn test_decode_2_padding() {
        assert_eq!(decode_base64("TQ=="), [77]);
    }

    #[test]
    fn test_encode_decode() {
        assert_eq!(
            decode_base64(&encode_base64(b"abcdefghiacbd")),
            b"abcdefghiacbd"
        );
    }

    #[test]
    fn test_encode_decode_emoji() {
        assert_eq!(
            decode_base64(&encode_base64("🦆🤏".as_bytes())),
            "🦆🤏".as_bytes()
        );
    }

    #[test]
    fn test_encode_decode_emoji_two() {
        assert_eq!(
            decode_base64(&encode_base64("😐😐😐".as_bytes())),
            "😐😐😐".as_bytes()
        );
    }

    #[test]
    fn test_encode_decode_emoji_three() {
        assert_eq!(
            decode_base64(&encode_base64("👩🏾‍🤝‍👩🏾👩🏻‍🤝‍🧑🏽👩‍👩‍👧".as_bytes())),
            "👩🏾‍🤝‍👩🏾👩🏻‍🤝‍🧑🏽👩‍👩‍👧".as_bytes()
        );
    }

    #[test]
    fn test_encode_1_char() {
        assert_eq!(encode_base64(b"."), "Lg==");
    }
    #[test]
    fn test_encode_2_char() {
        assert_eq!(encode_base64(b"Hi"), "SGk=");
    }
    #[test]
    fn test_encode_3_char() {
        assert_eq!(encode_base64(b"Man"), "TWFu");
    }
    #[test]
    fn test_encode_4_char() {
        assert_eq!(encode_base64(b"What"), "V2hhdA==");
    }
    #[test]
    fn test_encode_5_char() {
        assert_eq!(encode_base64(b"Hello"), "SGVsbG8=");
    }
    #[test]
    fn test_encode_6_char() {
        assert_eq!(encode_base64(b"Pickle"), "UGlja2xl");
    }
}
