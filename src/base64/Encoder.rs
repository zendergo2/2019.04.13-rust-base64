pub struct Encoder {
}

impl Encoder {
    pub fn encode(&self, input: &str) -> String {
        // create output string to hold adjusted characters
        // Every 3 bytes of input are are converted to 4 bytes of output
        let mut buf = String::with_capacity(input.len() * (4/3));
        // Explode input into individual characters to loop through
        let mut chars = input.chars();
        // Current character to process (8 bits)
        let mut ch: char = 'a';
        // Ho many buts to consume off of the input
        // Initially consume 6 bits at a time (converting this to base64 character)
        let mut bits_to_consume: u8 = 6;
        // "Inverse" of bits_to_consume, how many bits from the input to maintain
        let mut bits_to_mask: u8 = 2;
        
        let mut mask: u16 = 0;
        // used to choose 
        let mut swap: u8 = 0;
        let mut rust: u8 = 0;
        loop {
            match chars.next() {
                // If next character exists, set as ch
                Some(c) => ch = c,
                // No more characters to loop through
                None => {
                    // Whatever's left in rust gets output
                    buf.push(self.get_b64_char(rust));
                    // Add necessary padding
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
            if bits_to_consume == 0 {
                // This means we have a full "buffer", and need to do an extra pass to clear our buffer
                // We don't need to get a new character this time
                // Hoewever, we need to process an extra character
                buf.push(self.get_b64_char(rust));
                //println!("Finl2: {:?}", );

                // After consuming a character, we have 2 more characters left over
                // to process in this loop, and 8 new characters from ch.
                // So, we neeed to grap 4 bits off of ch and process, then put the rest in 
                // rust for next time. bits_to_consume will then be 4
                //rust = (rust << 6);
                swap = (ch as u8) >> 2;
                buf.push(self.get_b64_char(swap));
                println!("Finl2: {:?}", (swap, ch, self.get_b64_char(swap)));


                // Reset variables for next loop....
                rust = (ch as u8) << 2;
                bits_to_consume = 6;
                bits_to_mask = 8 - bits_to_consume;
                mask = (2 as u16).pow(bits_to_mask as u32) - 1;
                continue;
            }

            // First pass, shift 6 off of character. Second 4, etc.
            swap = (ch as u8) >> (8 - bits_to_consume);

            
            // XOR rust and swap, what do they do?
            buf.push(self.get_b64_char(rust | swap));

            // After consuming a character, we have 2 more characters left over
            // to process next time
            if bits_to_consume == 0 { bits_to_consume = 6; }
            else { bits_to_consume -= 2; }

            // binary mask increasing by 2 bits every loop, e.g. 11  > 1111 > 11111 > 0
            mask = (2 as u16).pow(bits_to_mask as u32) - 1;
            // the bits that need processing? Will do next loop I guess
            rust = ((ch as u8) & mask as u8) << bits_to_consume;

            // Refresh bits_to_mask
            bits_to_mask = 8 - bits_to_consume;
        }
        buf // final 64 bit string
    }

    // 6 bits -> equivalent 64bit character
    fn get_b64_char(&self, in_byte: u8) -> char {
        // println!("{:b} {}", in_byte, in_byte);
        if in_byte < 26 {
            // Capital letters (0-25 => 65-90)
            char::from(in_byte + 65)
        } else if in_byte < 52 {
            // Lowercase letters (26-51 => 97-122)
            char::from(in_byte + 71)
        } else if in_byte < 62 {
            // Numbers (52-61 => 48-57)
            char::from(in_byte - 4)
        } else if in_byte == 62 {
            // 62
            '/'
        } else if in_byte == 63 {
            // 63
            '+'
        } else {
            '#'
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wiki_1() {
        let enc = Encoder{};
        assert_eq!(enc.encode("Man"), "TWFu");
    }
    #[test]
    fn wiki_2() {
        let enc = Encoder{};
        assert_eq!(enc.encode("Ma"), "TWE=");
    }
    #[test]
    fn wiki_3() {
        let enc = Encoder{};
        assert_eq!(enc.encode("M"), "TQ==");
    }
    #[test]
    fn long_string() {
        let enc = Encoder{};
        assert_eq!(enc.encode("asdflkjwefoinvvofoifjdfjasdlvk"), "YXNkZmxrandlZm9pbnZ2b2ZvaWZqZGZqYXNkbHZr");
    }

}
