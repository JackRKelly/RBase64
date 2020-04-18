fn main() {
    println!("Hello, world!");
    let decode = "VGVzdA==";
    decode_base64(decode);
    let encode = vec![20, 30, 40, 50, 34, 60, 34, 30];
    encode_base64(encode);
}

fn decode_base64(base64: &str) {
    //let chunks = input.chars().map(|c| hashmap.get(c).unwrap()).chunks_exact(4);

    //Iterate over input string, compare string to list of indicies, chunk by groups of 4
    //[[20, 30, 40, 50], [34, 60, 34, 30]]

    //Convert to flat vector and return
}

fn encode_base64(bytes: Vec<u8>) {
    //Take flat vector of binary [01010101, 01010101, 01010101, 01010101, 01010101, 01010101]

    //Chunk flat vector in EXACT groups of 3 [[01010101, 01010101, 01010101], [01010101, 01010101, 01010101]]

    //Loop through parent vector
    //[[01010101, 01010101, 01010101]]

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
