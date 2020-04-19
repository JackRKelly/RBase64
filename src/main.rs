const CHARACTER_INDEX: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

fn main() {
    println!("Hello, world!");
    let decode = "VGVzdA==";
    decode_base64(decode);
    let encode = b"Man";
    dbg!(encode_base64(encode));
}

fn first_char(a: u8) -> u8 {
    (a & 0b1111_1100) >> 2
}

fn second_char(a: u8, b: u8) -> u8 {
    ((a & 0b0000_0011) << 4) | ((b & 0b1111_0000) >> 4)
}

fn third_char(a: u8, b: u8) -> u8 {
    ((a & 0b0000_1111) << 2) | ((b & 0b1100_0000) >> 6)
}

fn fourth_char(b: u8) -> u8 {
    b & 0b0011_1111
}

fn decode_base64(_base64: &str) {
    //let chunks = input.chars().map(|c| hashmap.get(c).unwrap()).chunks_exact(4);

    //Iterate over input string, compare string to list of indicies, chunk by groups of 4
    //[[20, 30, 40, 50], [34, 60, 34, 30]]

    //Convert to flat vector and return
}

fn encode_base64(bytes: &[u8]) -> String {
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
