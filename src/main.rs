fn main() {
    let mut to_encode = "Man";
    let mut expected_result = "TWFu";
    println!("{}, {} = {}", to_encode, encode(to_encode), expected_result);
    
    to_encode = "Ma";
    expected_result = "TWE=";
    println!("{}, {} = {}", to_encode, encode(to_encode), expected_result);

    to_encode = "M";
    expected_result = "TQ==";
    println!("{}, {} = {}", to_encode, encode(to_encode), expected_result);
}

// We need to take each character,
// consume 6 bytes, and consume the next character
// plus the 2 left over
fn encode(input: &str) -> String {
    // create output string to hold adjusted characters
    let mut buf = String::with_capacity(input.len());
    // keep around extra unconsumed bits
    /*
    let mut chars = input.chars(); //Get itr for characters in string
    let ch1 = chars.next().unwrap(); // 'M', 01001101
    let ch1_2 = (ch1 as u8) >> 2; // -> 010011, 19(T)
    let mut bitmask = 3; // 00000011
    let mut unconsumed = ((ch1 as u8) & bitmask) << 4; // 01001101 -> 01 -> 010000 
    let ch2 = chars.next().unwrap(); // 'a', 01100001
    let ch2_2 = unconsumed | ((ch2 as u8) >> 4); // 01100001 -> 0110 -> 010110, 22(W) 
    bitmask = 15; //00001111
    unconsumed = ((ch2 as u8) & bitmask) << 2; // 01100001 -> 0001 -> 000100
    let ch3 = chars.next().unwrap(); // 'n', 01101110
    let ch3_2 = unconsumed | ((ch3 as u8) >> 6); // 01101110 -> 01 -> 000101
    bitmask = 63; //00111111
    unconsumed = (ch3 as u8) & bitmask; // 01101110 -> 101110
    let ch4_2 = unconsumed;

    // Add new character to output string
    buf.push(get_b64_char(ch1_2));
    buf.push(get_b64_char(ch2_2));
    buf.push(get_b64_char(ch3_2));
    buf.push(get_b64_char(ch4_2));
    buf.push('|');
    */
    let mut chars = input.chars();
    let mut stop: bool = false;
    let mut ch: char = 'a';
    let mut bits_to_consume: u8 = 6;
    let mut bits_to_mask: u8 = 2;
    let mut mask: u8 = 0;
    let mut swap: u8 = 0;
    let mut rust: u8 = 0;
    while !stop {
        match chars.next() {
            Some(c) => ch = c,
            None => {
                buf.push(get_b64_char(rust)); // Whatever's left in rust gets output
                match bits_to_consume {
                    2 => buf.push('='), 
                    4 => {
                        buf.push('=');
                        buf.push('=');
                    },
                    _ => break,
                }
                break;
            },
        }
        // println!("BEGIN: {:?}", (mask, swap, rust, bits_to_consume, bits_to_mask));
        swap = (ch as u8) >> (((ch.len_utf8() as u8) * 8) - bits_to_consume);
        buf.push(get_b64_char(rust | swap));

        if bits_to_consume == 0 { bits_to_consume = 6; }
        else { bits_to_consume -= 2; }
        mask = (2 as u8).pow(bits_to_mask as u32) - 1;
        rust = ((ch as u8) & mask) << bits_to_consume; // bring back our dropped bits 
        bits_to_mask = 8 - bits_to_consume;
        // println!("  END: {:?}", (mask, swap, rust, bits_to_consume, bits_to_mask));
    }
    buf
}

fn get_b64_char(in_byte: u8) -> char {
    // println!("{:b} {}", in_byte, in_byte);
    if in_byte < 26 {
        // Capital letters
        char::from(in_byte + 65)
    } else if in_byte < 52 {
        // Lowercase letters
        char::from(in_byte + 71)
    } else if in_byte < 62 {
        // Numbers
        char::from(in_byte - 4)
    } else if in_byte == 63 {
        '/'
    } else {
        '+'
    }
}


