pub struct Encoder {
}

impl Encoder {
    pub fn encode(&self, input: &str) -> String {
        // create output string to hold adjusted characters
        let mut buf = String::with_capacity(input.len());
        let mut chars = input.chars();
        let mut stop = false;
        let mut ch = 'a';
        let mut bits_to_consume = 6;
        let mut bits_to_mask = 2;
        let mut mask = 0;
        let mut swap = 0;
        let mut rust = 0;
        while !stop {
            match chars.next() {
                Some(c) => ch = c,
                None => {
                    buf.push(self.get_b64_char(rust)); // Whatever's left in rust gets output
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
            buf.push(self.get_b64_char(rust | swap));

            if bits_to_consume == 0 { bits_to_consume = 6; }
            else { bits_to_consume -= 2; }
            mask = (2 as u8).pow(bits_to_mask as u32) - 1;
            rust = ((ch as u8) & mask) << bits_to_consume; // bring back our dropped bits 
            bits_to_mask = 8 - bits_to_consume;
            // println!("  END: {:?}", (mask, swap, rust, bits_to_consume, bits_to_mask));
        }
        buf
    }

    fn get_b64_char(&self, in_byte: u8) -> char {
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
}
