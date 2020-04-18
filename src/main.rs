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
    encode_base64(encode);
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

fn decode_base64(base64: &str) {
    //let chunks = input.chars().map(|c| hashmap.get(c).unwrap()).chunks_exact(4);

    //Iterate over input string, compare string to list of indicies, chunk by groups of 4
    //[[20, 30, 40, 50], [34, 60, 34, 30]]

    //Convert to flat vector and return
}

fn encode_base64(bytes: &[u8]) {
    //Take flat vector of binary [01010101, 01010101, 01010101, 01010101, 01010101, 01010101]

    //Chunk flat vector in EXACT groups of 3 [[01010101, 01010101, 01010101], [01010101, 01010101, 01010101]]
    println!("{:?}", bytes.chunks_exact(3));

    //Loop through parent vector
    //[[01010101, 01010101, 01010101]]

    let chunks = bytes.chunks_exact(3);
    for chunk in chunks {
        let first = CHARACTER_INDEX[usize::from(first_char(chunk[0]))];
        let second = CHARACTER_INDEX[usize::from(second_char(chunk[0], chunk[1]))];
        let third = CHARACTER_INDEX[usize::from(third_char(chunk[1], chunk[2]))];
        let fourth = CHARACTER_INDEX[usize::from(fourth_char(chunk[2]))];
        dbg!(chunk);
        dbg!(first, second, third, fourth);
    }

    //LOOP - through by
    //01010101 then 01010101 then 01010101
    //Step 1
    //Create array for first_char, second_char, third_char, fourth_char function results
    //Step 2
    //LOOP - Iterate over array of 4, 6 bit integers
    //Convert 6 bit integers into numerical composite values
    //010101 -> 21
    //Step 3
    //Match composite values against hashmap/list to get characters (max of 63)
    //Step 4
    //Add characters to final string variable

    //Handle EXACT group of 3 chunk remainder

    //Return final string variable
}
